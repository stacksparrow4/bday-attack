use std::{
    fs::File,
    io::{Read, Seek, Write},
};

use crate::{
    constants::HASH_TABLE_FILE_QUOTA,
    hash::{Hash, HashPair},
};

fn hash_to_index(hash: &Hash) -> u64 {
    return hash.to_u64() % num_entries();
}

pub const fn num_entries() -> u64 {
    HASH_TABLE_FILE_QUOTA / HashPair::size()
}

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

    pub fn insert_pair(&mut self, hash: HashPair) {
        let mut buf = [0u8; HashPair::size() as usize];

        let record_pos: u64 = HashPair::size() * hash_to_index(&hash.hash);
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

    pub fn flush(&mut self) {
        self.file.flush().unwrap();
    }
}

pub struct DiskHashMapReader {
    file: File,
}

impl DiskHashMapReader {
    pub fn new(fname: &'static str) -> Self {
        Self {
            file: File::open(fname).unwrap(),
        }
    }

    pub fn search(&mut self, hash: Hash) -> Option<HashPair> {
        let mut buf = [0u8; HashPair::size() as usize];

        let record_pos: u64 = HashPair::size() * hash_to_index(&hash);
        self.file
            .seek(std::io::SeekFrom::Start(record_pos))
            .unwrap();

        loop {
            self.file.read_exact(&mut buf).unwrap();
            let hp = HashPair::from_bytes(&buf);

            if hp.num_spaces == 0 {
                return None;
            } else if hp.hash == hash {
                return Some(hp);
            }
        }
    }
}
