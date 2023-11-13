use crate::constants::{NumSpacesType, DESIRED_HEX_MATCHES, NUM_HASH_BYTES, SHA_BLOCK_SIZE};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Hash {
    pub(crate) data: [u8; NUM_HASH_BYTES],
}

impl Hash {
    pub(crate) fn from_full_hash(full_hash: [u8; 32]) -> Self {
        let mut data: [u8; NUM_HASH_BYTES] = full_hash[(32 - NUM_HASH_BYTES)..].try_into().unwrap();
        if DESIRED_HEX_MATCHES % 2 == 1 {
            data[0] &= 0x0f;
        }
        Self { data }
    }
}

pub(crate) struct HashPair {
    pub(crate) hash: Hash,
    pub(crate) num_spaces: NumSpacesType,
}

impl HashPair {
    pub(crate) fn new(hash: Hash, num_spaces: NumSpacesType) -> Self {
        Self { hash, num_spaces }
    }
}

pub(crate) type HashBatch = [HashPair; SHA_BLOCK_SIZE as usize];
