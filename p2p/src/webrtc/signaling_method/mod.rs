//! WebRTC Signaling Transport Methods
//!
//! This module defines the different transport methods available for WebRTC signaling
//! in OpenMina's peer-to-peer network. WebRTC requires an external signaling mechanism
//! to exchange connection metadata before establishing direct peer-to-peer connections.
//!
//! ## Signaling Transport Methods
//!
//! OpenMina supports multiple signaling transport methods to accommodate different
//! network environments and security requirements:
//!
//! ### HTTP/HTTPS Direct Connections
//!
//! - **HTTP**: Direct HTTP connections to signaling servers (typically for local/testing)
//! - **HTTPS**: Secure HTTPS connections to signaling servers (recommended for production)
//!
//! These methods allow peers to directly contact signaling servers to exchange offers
//! and answers for WebRTC connection establishment.
//!
//! ### HTTPS Proxy
//!
//! - **HTTPS Proxy**: Uses an SSL gateway/proxy server to reach the actual signaling server
//!
//! ### P2P Relay Signaling
//!
//! - **P2P Relay**: Uses existing peer connections to relay signaling messages
//! - Enables signaling through already-established peer connections
//! - Provides redundancy when direct signaling server access is unavailable
//! - Supports bootstrapping new connections through existing network peers
//!
//! ## URL Format
//!
//! Signaling methods use a structured URL format:
//!
//! - HTTP: `/http/{host}/{port}`
//! - HTTPS: `/https/{host}/{port}`
//! - HTTPS Proxy: `/https_proxy/{cluster_id}/{host}/{port}`
//! - P2P Relay: `/p2p/{peer_id}`
//!
//! ## Connection Strategy
//!
//! The signaling method determines how peers discover and connect to each other:
//!
//! 1. **Direct Methods** (HTTP/HTTPS) - Can connect immediately to signaling servers
//! 2. **Proxy Methods** - Route through intermediate proxy infrastructure
//! 3. **Relay Methods** - Require existing peer connections for message routing

mod http;
pub use http::HttpSignalingInfo;

use std::{fmt, str::FromStr};

use binprot_derive::{BinProtRead, BinProtWrite};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::PeerId;

/// WebRTC signaling transport method configuration.
///
/// `SignalingMethod` defines how WebRTC signaling messages (offers and answers)
/// are transported between peers. Different methods provide flexibility for
/// various network environments and infrastructure requirements.
///
/// # Method Types
///
/// - **HTTP/HTTPS**: Direct connections to signaling servers
/// - **HTTPS Proxy**: Connections through SSL gateway/proxy servers
/// - **P2P Relay**: Signaling through existing peer connections
///
/// Each method encapsulates the necessary connection information to establish
/// the signaling channel, which is used before the actual WebRTC peer-to-peer
/// connection is established.
///
/// # Usage
///
/// Signaling methods can be parsed from string representations or constructed
/// programmatically. They support serialization for storage and network transmission.
///
/// # Example
///
/// ```
/// // Direct HTTPS signaling
/// let method = "/https/signal.example.com/443".parse::<SignalingMethod>()?;
///
/// // P2P relay through an existing peer
/// let method = SignalingMethod::P2p { relay_peer_id: peer_id };
/// ```
#[derive(BinProtWrite, BinProtRead, Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub enum SignalingMethod {
    /// HTTP signaling server connection.
    ///
    /// Uses plain HTTP for signaling message exchange. Typically used for
    /// local development or testing environments where encryption is not required.
    Http(HttpSignalingInfo),

    /// HTTPS signaling server connection.
    ///
    /// Uses secure HTTPS for signaling message exchange. Recommended for
    /// production environments to protect signaling data in transit.
    Https(HttpSignalingInfo),

    /// HTTPS proxy signaling connection.
    ///
    /// Uses an SSL gateway/proxy server to reach the actual signaling server.
    /// The first parameter is the cluster ID for routing, and the second
    /// parameter contains the proxy server connection information.
    ///
    /// This method supports cluster-based routing for scalable signaling
    /// infrastructure and is useful in enterprise environments.
    HttpsProxy(u16, HttpSignalingInfo),

    /// P2P relay signaling through an existing peer connection.
    ///
    /// Uses an already-established peer connection to relay signaling messages
    /// to other peers. This enables signaling when direct access to signaling
    /// servers is unavailable and provides redundancy in the signaling process.
    P2p {
        /// The peer ID of the relay peer that will forward signaling messages.
        relay_peer_id: PeerId,
    },
}

impl SignalingMethod {
    /// Determines if this signaling method supports direct connections.
    ///
    /// Direct connection methods (HTTP, HTTPS, HTTPS Proxy) can establish
    /// signaling channels immediately without requiring existing peer connections.
    /// P2P relay methods require an already-established peer connection to function.
    ///
    /// # Returns
    ///
    /// * `true` for HTTP, HTTPS, and HTTPS Proxy methods
    /// * `false` for P2P relay methods
    ///
    /// This is useful for connection strategy decisions and determining whether
    /// bootstrap connections are needed before signaling can occur.
    pub fn can_connect_directly(&self) -> bool {
        match self {
            Self::Http(_) | Self::Https(_) | Self::HttpsProxy(_, _) => true,
            Self::P2p { .. } => false,
        }
    }

    /// Constructs the HTTP(S) URL for sending WebRTC offers.
    ///
    /// This method generates the appropriate URL endpoint for sending WebRTC
    /// signaling messages based on the signaling method configuration.
    ///
    /// # URL Formats
    ///
    /// - **HTTP**: `http://{host}:{port}/mina/webrtc/signal`
    /// - **HTTPS**: `https://{host}:{port}/mina/webrtc/signal`
    /// - **HTTPS Proxy**: `https://{host}:{port}/clusters/{cluster_id}/mina/webrtc/signal`
    ///
    /// # Returns
    ///
    /// * `Some(String)` containing the signaling URL for HTTP-based methods
    /// * `None` for P2P relay methods that don't use HTTP endpoints
    ///
    /// # Example
    ///
    /// ```
    /// let method = SignalingMethod::Https(info);
    /// let url = method.http_url(); // Some("https://signal.example.com:443/mina/webrtc/signal")
    /// ```
    pub fn http_url(&self) -> Option<String> {
        let (http, info) = match self {
            Self::Http(info) => ("http", info),
            Self::Https(info) => ("https", info),
            Self::HttpsProxy(cluster_id, info) => {
                return Some(format!(
                    "https://{}:{}/clusters/{}/mina/webrtc/signal",
                    info.host, info.port, cluster_id
                ));
            }
            _ => return None,
        };
        Some(format!(
            "{http}://{}:{}/mina/webrtc/signal",
            info.host, info.port,
        ))
    }

    /// Extracts the relay peer ID for P2P signaling methods.
    ///
    /// For P2P relay signaling methods, this returns the peer ID of the
    /// intermediate peer that will forward signaling messages. This is used
    /// to identify which existing peer connection should be used for relaying.
    ///
    /// # Returns
    ///
    /// * `Some(PeerId)` for P2P relay methods
    /// * `None` for direct connection methods (HTTP/HTTPS)
    ///
    /// # Usage
    ///
    /// This method is typically used when setting up message routing for
    /// P2P relay signaling to determine which peer connection should handle
    /// the signaling traffic.
    pub fn p2p_relay_peer_id(&self) -> Option<PeerId> {
        match self {
            Self::P2p { relay_peer_id } => Some(*relay_peer_id),
            _ => None,
        }
    }
}

impl fmt::Display for SignalingMethod {
    /// Formats the signaling method as a URL path string.
    ///
    /// This implementation converts the signaling method into its string
    /// representation following the URL format patterns. The formatted
    /// string can be parsed back using [`FromStr`].
    ///
    /// # Format Patterns
    ///
    /// - HTTP: `/http/{host}/{port}`
    /// - HTTPS: `/https/{host}/{port}`
    /// - HTTPS Proxy: `/https_proxy/{cluster_id}/{host}/{port}`
    /// - P2P Relay: `/p2p/{peer_id}`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(signaling) => {
                write!(f, "/http")?;
                signaling.fmt(f)
            }
            Self::Https(signaling) => {
                write!(f, "/https")?;
                signaling.fmt(f)
            }
            Self::HttpsProxy(cluster_id, signaling) => {
                write!(f, "/https_proxy/{cluster_id}")?;
                signaling.fmt(f)
            }
            Self::P2p { relay_peer_id } => {
                write!(f, "/p2p/{relay_peer_id}")
            }
        }
    }
}

/// Errors that can occur when parsing signaling method strings.
///
/// `SignalingMethodParseError` provides detailed error information for
/// parsing failures when converting string representations to [`SignalingMethod`]
/// instances. This helps with debugging configuration and user input validation.
///
/// # Error Types
///
/// The parser can fail for various reasons including missing components,
/// invalid formats, or unsupported method types. Each error variant provides
/// specific context about what went wrong during parsing.
#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum SignalingMethodParseError {
    /// Insufficient arguments provided for the signaling method.
    ///
    /// This occurs when the input string doesn't contain enough components
    /// to construct a valid signaling method. For example, missing host
    /// or port information for HTTP methods.
    #[error("not enough args for the signaling method")]
    NotEnoughArgs,

    /// Unknown or unsupported signaling method type.
    ///
    /// This occurs when the method type (first component) is not recognized.
    /// Supported methods are: `http`, `https`, `https_proxy`, `p2p`.
    #[error("unknown signaling method: `{0}`")]
    UnknownSignalingMethod(String),

    /// Invalid cluster ID for HTTPS proxy methods.
    ///
    /// This occurs when the cluster ID component cannot be parsed as a
    /// valid 16-bit unsigned integer for HTTPS proxy configurations.
    #[error("invalid cluster id")]
    InvalidClusterId,

    /// Failed to parse the host component.
    ///
    /// This occurs when the host string cannot be parsed as a valid
    /// hostname, IP address, or multiaddr format by the Host parser.
    #[error("host parse error: {0}")]
    HostParseError(String),

    /// Failed to parse the port component.
    ///
    /// This occurs when the port string cannot be parsed as a valid
    /// 16-bit unsigned integer port number.
    #[error("port parse error: {0}")]
    PortParseError(String),
}

impl FromStr for SignalingMethod {
    type Err = SignalingMethodParseError;

    /// Parses a string representation into a [`SignalingMethod`].
    ///
    /// This method parses URL-like strings that represent different signaling
    /// transport methods. The parser supports the following formats:
    ///
    /// # Supported Formats
    ///
    /// - **HTTP**: `/http/{host}/{port}`
    /// - **HTTPS**: `/https/{host}/{port}`
    /// - **HTTPS Proxy**: `/https_proxy/{cluster_id}/{host}/{port}`
    /// - **P2P Relay**: `/p2p/{peer_id}`
    ///
    /// # Examples
    ///
    /// ```
    /// use openmina::signaling_method::SignalingMethod;
    ///
    /// // HTTP signaling
    /// let method: SignalingMethod = "/http/localhost/8080".parse()?;
    ///
    /// // HTTPS signaling
    /// let method: SignalingMethod = "/https/signal.example.com/443".parse()?;
    ///
    /// // HTTPS proxy with cluster ID
    /// let method: SignalingMethod = "/https_proxy/123/proxy.example.com/443".parse()?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`SignalingMethodParseError`] for various parsing failures:
    /// - Missing components (host, port, etc.)
    /// - Unknown method types
    /// - Invalid numeric values (ports, cluster IDs)
    /// - Invalid host formats
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(SignalingMethodParseError::NotEnoughArgs);
        }

        let method_end_index = s[1..]
            .find('/')
            .map(|i| i + 1)
            .filter(|i| s.len() > *i)
            .ok_or(SignalingMethodParseError::NotEnoughArgs)?;

        let rest = &s[method_end_index..];
        match &s[1..method_end_index] {
            "http" => Ok(Self::Http(rest.parse()?)),
            "https" => Ok(Self::Https(rest.parse()?)),
            "https_proxy" => {
                let mut iter = rest.splitn(3, '/').filter(|v| !v.trim().is_empty());
                let (cluster_id, rest) = (
                    iter.next()
                        .ok_or(SignalingMethodParseError::NotEnoughArgs)?,
                    iter.next()
                        .ok_or(SignalingMethodParseError::NotEnoughArgs)?,
                );
                let cluster_id: u16 = cluster_id
                    .parse()
                    .or(Err(SignalingMethodParseError::InvalidClusterId))?;
                Ok(Self::HttpsProxy(cluster_id, rest.parse()?))
            }
            method => Err(SignalingMethodParseError::UnknownSignalingMethod(
                method.to_owned(),
            )),
        }
    }
}

impl Serialize for SignalingMethod {
    /// Serializes the signaling method as a string.
    ///
    /// This uses the `Display` implementation to convert the signaling
    /// method to its string representation for serialization.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for SignalingMethod {
    /// Deserializes a signaling method from a string.
    ///
    /// This uses the [`FromStr`] implementation to parse the string
    /// representation back into a [`SignalingMethod`] instance.
    ///
    /// # Errors
    ///
    /// Returns a deserialization error if the string cannot be parsed
    /// as a valid signaling method.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
