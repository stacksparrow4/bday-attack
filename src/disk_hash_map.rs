use std::{
    fs::File,
    io::{Read, Seek, Write},
};

use crate::{
    constants::HASH_TABLE_FILE_QUOTA,
    hash::{Hash, HashPair},
};

pub struct DiskHashMapWriter {
    file: File,
}

impl DiskHashMapWriter {
    pub fn new(fname: &'static str) -> Self {
        let mut f = File::options()
            .write(true)
            .read(true)
            .create(true)
            .open(fname)
            .unwrap();
        f.seek(std::io::SeekFrom::Start(HASH_TABLE_FILE_QUOTA))
            .unwrap();
        f.write_all(&[0u8]).unwrap();

        Self { file: f }
    }

    fn hash_to_index(hash: &Hash) -> u64 {
        return hash.to_u64() % Self::num_entries();
    }

    pub fn insert_pair(&mut self, hash: HashPair) {
        let mut buf = [0u8; HashPair::size() as usize];

        let record_pos: u64 = HashPair::size() * Self::hash_to_index(&hash.hash);
        self.file
            .seek(std::io::SeekFrom::Start(record_pos))
            .unwrap();

        loop {
            self.file.read_exact(&mut buf).unwrap();
            if HashPair::from_bytes(&buf).num_spaces == 0 {
                break;
            }
        }

        self.file
            .seek(std::io::SeekFrom::Current(-(HashPair::size() as i64)))
            .unwrap();
        self.file.write_all(&hash.to_bytes()).unwrap();
    }

    pub const fn num_entries() -> u64 {
        HASH_TABLE_FILE_QUOTA / HashPair::size()
    }
}
