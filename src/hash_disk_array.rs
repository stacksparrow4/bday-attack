use std::{
    fs::File,
    io::{Read, Seek, Write},
    mem::size_of,
};

use crate::{constants::NUM_HASH_BYTES, hash_gen::HashPair};

pub struct HashDiskArray {
    file: File,
}

impl HashDiskArray {
    pub fn new(fname: &'static str) -> Self {
        Self {
            file: File::options().read(true).write(true).open(fname).unwrap(),
        }
    }

    pub fn add_hash(&mut self, hash: HashPair) {
        self.file.write_all(&hash.0).unwrap();
        self.file.write_all(&usize::to_le_bytes(hash.1)).unwrap();
    }
}

impl IntoIterator for HashDiskArray {
    type Item = HashPair;

    type IntoIter = HashDiskArrayIterator;

    fn into_iter(mut self) -> Self::IntoIter {
        self.file.seek(std::io::SeekFrom::Start(0u64)).unwrap();
        HashDiskArrayIterator { hda: self }
    }
}

pub struct HashDiskArrayIterator {
    hda: HashDiskArray,
}

impl Iterator for HashDiskArrayIterator {
    type Item = HashPair;

    fn next(&mut self) -> Option<Self::Item> {
        let mut data = [0u8; NUM_HASH_BYTES + size_of::<usize>()];
        match self.hda.file.read_exact(&mut data) {
            Ok(_) => Some((
                data[..NUM_HASH_BYTES].try_into().unwrap(),
                usize::from_le_bytes(data[NUM_HASH_BYTES..].try_into().unwrap()),
            )),
            Err(_) => None,
        }
    }
}
