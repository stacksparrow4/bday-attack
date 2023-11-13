// Configuration
pub const NUM_HASHES: NumSpacesType = 1 * 1000000;
pub const DESIRED_HEX_MATCHES: usize = 8;

// Optimization
pub const HASH_GEN_WORKER_THREADS: usize = 16;
pub const HASH_SEARCH_WORKER_THREADS: usize = 16;
pub const CHANNEL_SIZE: usize = 100 * 1024;

// Calculated
pub const NUM_HASH_BYTES: usize = (DESIRED_HEX_MATCHES + 1) / 2;

// Fixed
pub type NumSpacesType = u32;
pub const SHA_BLOCK_SIZE: NumSpacesType = 64;
