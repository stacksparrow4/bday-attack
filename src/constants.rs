// Configuration
pub(crate) const NUM_HASHES: LineMaskType = 4000 * 1000000;
pub(crate) const DESIRED_HEX_MATCHES: usize = 16;

pub(crate) const PREHASH_SIZE: usize = 36;

// Optimization
pub(crate) const NUM_THREADS: LineMaskType = 8;
pub(crate) const CHANNEL_SIZE: usize = 100 * 1024;

// Calculated
pub(crate) const NUM_HASH_BYTES: usize = (DESIRED_HEX_MATCHES + 1) / 2;

// Fixed
pub(crate) type LineMaskType = usize;
