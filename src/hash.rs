use crate::constants::{NumSpacesType, DESIRED_HEX_MATCHES, NUM_HASH_BYTES};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
}

pub(crate) type HashLastDigitsPair = (HashLastDigits, NumSpacesType);
