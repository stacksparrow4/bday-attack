use crate::hash::HashPair;

// Configuration
pub const NUM_HASHES: NumSpacesType = 1 * 1000000;
pub const HASH_GEN_WORKER_THREADS: usize = 16;
pub const DESIRED_HEX_MATCHES: usize = 8;
pub const HASH_TABLE_DENSITY: f32 = 0.5;

pub type NumSpacesType = u32;

// Calculated
pub const NUM_HASH_BYTES: usize = (DESIRED_HEX_MATCHES + 1) / 2;
pub const HASH_TABLE_FILE_QUOTA: u64 =
    (((HashPair::size() * (NUM_HASHES as u64)) as f32) / HASH_TABLE_DENSITY) as u64;

// Fixed
pub const SHA_BLOCK_SIZE: NumSpacesType = 64;
