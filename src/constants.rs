// Configuration
pub(crate) const NUM_HASHES: LineMaskType = 1000000;
pub(crate) const DESIRED_HEX_MATCHES: usize = 9;

pub(crate) const PREHASH_SIZE: usize = 35;

// Optimization
pub(crate) const THREADED_BITS: usize = 3; // num threads = 2^THREADED_BITS
pub(crate) const CHANNEL_SIZE: usize = 100 * 1024;

// Calculated
pub(crate) const NUM_THREADS: LineMaskType = 2u32.pow(THREADED_BITS as u32);
pub(crate) const NUM_HASH_BYTES: usize = (DESIRED_HEX_MATCHES + 1) / 2;

// Fixed
pub(crate) type LineMaskType = u32;
