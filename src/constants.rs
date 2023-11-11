// Configuration
pub const NUM_HASHES: NumSpacesType = 40 * 1000000;
pub const HASH_GEN_WORKER_THREADS: usize = 16;
pub const DESIRED_HEX_MATCHES: usize = 15;
pub const NUM_HASH_BYTES: usize = (DESIRED_HEX_MATCHES + 1) / 2;

pub const HASH_TABLE_FILE_QUOTA: u64 = 1 * 1024 * 1024 * 1024; // 1gb

pub type NumSpacesType = u32;

// Fixed
pub const SHA_BLOCK_SIZE: NumSpacesType = 64;
