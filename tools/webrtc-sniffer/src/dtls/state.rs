use std::borrow::Cow;

use nom::{multi::many1, IResult, Parser};
use rand::{rngs::StdRng, SeedableRng};

use crate::dtls::handshake::{HandshakeInner, ServerHello};

use super::{
    handshake::{Extension, HandshakeMessage},
    header::{Chunk, ContentType},
};

pub struct State {
    initial_seed: [u8; 32],
    final_seed: [u8; 32],
    inner: Option<Inner>,
}

enum Inner {
    Initial,
    ClientHello {
        client_random: [u8; 32],
    },
    BothHello(HelloMsgs),
    ServerKey {
        hello: HelloMsgs,
        // https://www.iana.org/assignments/tls-parameters/tls-parameters.xhtml#tls-parameters-8
        curve_name: u16,
        server_pk: Vec<u8>,
    },
    BothKey {
        hello: HelloMsgs,
        keys: BothKey,
    },
}

#[allow(dead_code)]
struct HelloMsgs {
    client_random: [u8; 32],
    server_random: [u8; 32],
    session_id: Vec<u8>,
    cipher_suite: u16,
    extensions: Vec<Extension>,
}

#[allow(dead_code)]
struct BothKey {
    curve_name: u16,
    server_pk: Vec<u8>,
    client_pk: Vec<u8>,
}

impl State {
    pub fn new(initial_seed: [u8; 32]) -> Self {
        State {
            initial_seed,
            final_seed: [0; 32],
            inner: Some(Inner::Initial),
        }
    }

    pub fn handle<'d>(&mut self, data: &'d [u8], _incoming: bool) -> IResult<&'d [u8], ()> {
        let (data, chunks) = many1(Chunk::parse).parse(data)?;
        for chunk in chunks {
            log::info!("{chunk}");
            #[allow(clippy::single_match)]
            match chunk.ty {
                ContentType::Handshake => self.handle_handshake(chunk.body),
                _ => {}
            }
        }

        Ok((data, ()))
    }

    fn handle_handshake(&mut self, msg_bytes: &[u8]) {
        let Some(state) = self.inner.take() else {
            log::warn!("ignore datagram, invalid state");
            return;
        };

        let mut msg_bytes = Cow::Borrowed(msg_bytes);
        if let Inner::BothKey { hello, keys } = &state {
            let bytes = msg_bytes.to_mut();
            // decrypt
            let _ = (hello, keys, bytes);
        }
        let msg = match HandshakeMessage::parse(&msg_bytes) {
            Ok((_, msg)) => msg,
            Err(err) => {
                log::error!("{err}");
                return;
            }
        };

        let HandshakeMessage {
            length,
            message_seq,
            fragment_offset,
            fragment_length,
            inner: msg,
        } = msg;
        let _ = message_seq;
        log::info!("HANDSHAKE: {msg}");

        if fragment_offset != 0 || length != fragment_length {
            log::error!("collecting fragments is not implemented");
            self.inner = None;
            return;
        }

        let state = match (state, msg) {
            (Inner::Initial, HandshakeInner::ClientHello(msg)) => {
                let client_random = msg.random;
                if msg.cookie.is_empty() {
                    self.inner = Some(Inner::Initial);
                    return;
                }

                use sha2::{
                    digest::{FixedOutput, Update},
                    Sha256,
                };
                self.final_seed = Sha256::default()
                    .chain(self.initial_seed)
                    .chain(&msg.cookie)
                    .finalize_fixed()
                    .into();

                let _ = (
                    msg.session_id,
                    msg.cookie,
                    msg.cipher_suites,
                    msg.compression_methods,
                    msg.extensions,
                );
                Inner::ClientHello { client_random }
            }
            (Inner::ClientHello { client_random }, HandshakeInner::ServerHello(msg)) => {
                let ServerHello {
                    random,
                    session_id,
                    cipher_suite,
                    compression_method,
                    extensions,
                } = msg;
                if compression_method != 0 {
                    log::error!("compression method {compression_method} is not implemented");
                    return;
                }
                Inner::BothHello(HelloMsgs {
                    client_random,
                    server_random: random,
                    session_id,
                    cipher_suite,
                    extensions,
                })
            }
            (Inner::BothHello(hello), HandshakeInner::ServerKeyExchange(msg)) => {
                // check signature
                let _ = msg.signature;
                Inner::ServerKey {
                    hello,
                    curve_name: msg.curve_name,
                    server_pk: msg.public_key,
                }
            }
            (
                Inner::ServerKey {
                    hello,
                    curve_name,
                    server_pk,
                },
                HandshakeInner::ClientKeyExchange(msg),
            ) => {
                let mut keys = BothKey {
                    curve_name,
                    server_pk,
                    client_pk: msg.public_key,
                };
                let _pre_master_secret = keys.compute_pre_master_secret(self.final_seed).unwrap();
                log::info!("pre_master_secret={}", hex::encode(_pre_master_secret));
                Inner::BothKey { hello, keys }
            }
            (state, _) => {
                log::warn!("ignore handshake msg");
                state
            }
        };
        self.inner = Some(state);
    }
}

impl BothKey {
    fn compute_pre_master_secret(&mut self, seed: [u8; 32]) -> anyhow::Result<Vec<u8>> {
        let mut rng = StdRng::from_seed(seed);
        let secret = match self.curve_name {
            23 => ss::ss::<p256::NistP256>(&mut rng, &self.client_pk, &self.server_pk)?,
            24 => ss::ss::<p384::NistP384>(&mut rng, &self.client_pk, &self.server_pk)?,
            29 => {
                let secret_key = x25519_dalek::StaticSecret::random_from_rng(rng);
                let public_key = x25519_dalek::PublicKey::from(&secret_key);
                let public_key_bytes = public_key.as_bytes();

                let pub_key: [u8; 32] = if self.client_pk.eq(public_key_bytes.as_ref()) {
                    self.server_pk
                        .as_slice()
                        .try_into()
                        .map_err(|_| anyhow::anyhow!("wrong size of pk"))?
                } else if self.server_pk.eq(public_key_bytes.as_ref()) {
                    self.client_pk
                        .as_slice()
                        .try_into()
                        .map_err(|_| anyhow::anyhow!("wrong size of pk"))?
                } else {
                    return Err(anyhow::anyhow!("missing correct seed"));
                };

                let other_public_key = x25519_dalek::PublicKey::from(pub_key);
                secret_key
                    .diffie_hellman(&other_public_key)
                    .as_bytes()
                    .to_vec()
            }
            c => return Err(anyhow::anyhow!("curve {c} is not supported")),
        };

        Ok(secret)
    }
}

mod ss {
    use elliptic_curve::{
        ecdh::EphemeralSecret,
        point::PointCompression,
        rand_core::CryptoRngCore,
        sec1::{EncodedPoint, FromEncodedPoint, ModulusSize, ToEncodedPoint},
        Curve, CurveArithmetic, PublicKey,
    };

    pub fn ss<C>(
        rng: &mut impl CryptoRngCore,
        client_pk: &[u8],
        server_pk: &[u8],
    ) -> anyhow::Result<Vec<u8>>
    where
        C: CurveArithmetic + Curve + PointCompression,
        <C as Curve>::FieldBytesSize: ModulusSize,
        <C as CurveArithmetic>::AffinePoint: FromEncodedPoint<C> + ToEncodedPoint<C>,
    {
        let secret_key = EphemeralSecret::<C>::random(rng);
        let public_key = EncodedPoint::<C>::from(secret_key.public_key());
        let public_key_bytes = public_key.to_bytes();

        let other_public_key = if client_pk.eq(public_key_bytes.as_ref()) {
            PublicKey::<C>::from_sec1_bytes(&server_pk)?
        } else if server_pk.eq(public_key_bytes.as_ref()) {
            PublicKey::<C>::from_sec1_bytes(&client_pk)?
        } else {
            return Err(anyhow::anyhow!("missing correct seed"));
        };
        Ok(secret_key
            .diffie_hellman(&other_public_key)
            .raw_secret_bytes()
            .to_vec())
    }
}
