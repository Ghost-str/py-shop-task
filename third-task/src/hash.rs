use sha2::{
    digest::{
        generic_array::GenericArray,
        typenum::{
            bit::{B0, B1},
            UInt, UTerm,
        },
    },
    Digest, Sha256,
};
use std::{fmt::Display, ops::Index};

#[derive(Debug)]
pub struct Hash(
    GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>,
);

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl Index<u8> for Hash {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl From<u128> for Hash {
    fn from(value: u128) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(value.to_be_bytes());
        Hash(hasher.finalize())
    }
}
