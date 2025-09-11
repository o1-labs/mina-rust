use mina_curves::pasta::Fp;

use crate::bigint::InvalidBigInt;

pub trait MinaHash {
    fn try_hash(&self) -> Result<Fp, InvalidBigInt>;
}
