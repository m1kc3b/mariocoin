use crate::U256;
use sha256::digest;
use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Hash(U256);

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl Hash {
    pub fn hash<T: Serialize>(data: &T) -> Self {
        let mut serialized: Vec<u8> = vec![];

        if let Err(e) = ciborium::into_writer(data, &mut serialized) {
            panic!("Failed to serialize data {:?}. This should not happen", e);
        }

        let hash = digest(&serialized);
        let hash_bytes = hex::decode(hash).unwrap();
        let hash_array: [u8; 32] = hash_bytes.as_slice()
            .try_into()
            .unwrap();

        Hash(U256::from_big_endian(&hash_array))
    }

    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }

    pub fn zero() -> Self {
        Hash(U256::zero())
    }

    pub fn as_bytes(&self) -> [u8; 32] {
        self.0.to_little_endian()
    }
}