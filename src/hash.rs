use crate::constants::{NumSpacesType, DESIRED_HEX_MATCHES, NUM_HASH_BYTES};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash {
    data: [u8; NUM_HASH_BYTES],
}

impl Hash {
    pub fn from_full_hash(full_hash: [u8; 32]) -> Self {
        let mut data: [u8; NUM_HASH_BYTES] = full_hash[(32 - NUM_HASH_BYTES)..].try_into().unwrap();
        data.reverse();
        if DESIRED_HEX_MATCHES % 2 == 1 {
            data[0] &= 0x0f;
        }
        Self { data }
    }
}

pub struct HashPair {
    pub hash: Hash,
    pub num_spaces: NumSpacesType,
}

impl HashPair {
    pub fn new(hash: Hash, num_spaces: NumSpacesType) -> Self {
        Self { hash, num_spaces }
    }
}
