use std::mem::size_of;

use crate::constants::{NumSpacesType, NUM_HASH_BYTES};

#[derive(PartialEq)]
pub struct Hash {
    data: [u8; NUM_HASH_BYTES],
}

impl Hash {
    pub fn new(data: [u8; NUM_HASH_BYTES]) -> Self {
        Self { data }
    }

    pub fn from_full_hash(full_hash: [u8; 32]) -> Self {
        // Hardcoding for efficiency
        Self {
            data: [
                full_hash[31],
                full_hash[30],
                full_hash[29],
                full_hash[28],
                full_hash[27],
                full_hash[26],
            ],
        }
        // Self {
        //     data: [
        //         full_hash[31],
        //         full_hash[30],
        //         full_hash[29],
        //         full_hash[28],
        //         full_hash[27],
        //         full_hash[26],
        //         full_hash[25],
        //         full_hash[24] & 0x0f,
        //     ],
        // }
    }

    pub fn to_u64(&self) -> u64 {
        let mut buf = [0u8; 8];
        buf[..NUM_HASH_BYTES].copy_from_slice(&self.data);
        u64::from_le_bytes(buf)
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

    pub fn to_bytes(&self) -> Vec<u8> {
        Vec::from_iter(
            self.hash
                .data
                .iter()
                .chain(NumSpacesType::to_le_bytes(self.num_spaces).iter())
                .map(|x| *x),
        )
    }

    pub fn from_bytes(b: &[u8]) -> HashPair {
        HashPair::new(
            Hash::new(b[..NUM_HASH_BYTES].try_into().unwrap()),
            NumSpacesType::from_le_bytes(b[NUM_HASH_BYTES..].try_into().unwrap()),
        )
    }

    pub const fn size() -> u64 {
        (NUM_HASH_BYTES + size_of::<NumSpacesType>()) as u64
    }
}
