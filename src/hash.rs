use crate::constants::{LineMaskType, DESIRED_HEX_MATCHES, NUM_HASH_BYTES, PREHASH_SIZE};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct HashLastDigits {
    data: [u8; NUM_HASH_BYTES],
}

impl HashLastDigits {
    pub(crate) fn from_full_hash(full_hash: [u8; 32]) -> Self {
        let mut data: [u8; NUM_HASH_BYTES] = full_hash[(32 - NUM_HASH_BYTES)..].try_into().unwrap();
        if DESIRED_HEX_MATCHES % 2 == 1 {
            data[0] &= 0x0f;
        }
        Self { data }
    }

    pub(crate) fn prehash(&self) -> usize {
        let mut new_data = [0u8; 8];
        new_data[..NUM_HASH_BYTES].copy_from_slice(&self.data);
        usize::from_le_bytes(new_data) & ((1 << PREHASH_SIZE) - 1)
    }
}

pub(crate) type HashLastDigitsPair = (HashLastDigits, LineMaskType);
