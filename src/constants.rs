// Configuration
pub(crate) const NUM_HASHES: NumSpacesType = 1 * 1000000;
pub(crate) const DESIRED_HEX_MATCHES: usize = 10;

// Optimization
pub(crate) const HASH_GEN_WORKER_THREADS: usize = 8;
pub(crate) const HASH_SEARCH_WORKER_THREADS: usize = 8;
pub(crate) const CHANNEL_SIZE: usize = 100 * 1024;

// Calculated
pub(crate) const NUM_HASH_BYTES: usize = (DESIRED_HEX_MATCHES + 1) / 2;

// Fixed
pub(crate) type NumSpacesType = u32;
pub(crate) const SHA_BLOCK_SIZE: NumSpacesType = 64;
