use std::{
    sync::mpsc::{self, Receiver},
    thread,
};

use crate::{
    constants::{NumSpacesType, HASH_GEN_WORKER_THREADS, SHA_BLOCK_SIZE},
    hash::{Hash, HashPair},
    sha::Sha256,
};

fn get_hashes_for_one_block(state: Sha256, num_spaces: NumSpacesType) -> Vec<HashPair> {
    (0..SHA_BLOCK_SIZE)
        .map(|i| {
            let mut s = state.clone();
            s.update(&b" ".repeat(i as usize));

            let full_hash = s.finish();

            HashPair::new(Hash::from_full_hash(full_hash), num_spaces + i)
        })
        .collect()
}

pub fn get_reversed_hashes(
    start_str: &'static str,
    num_hashes: NumSpacesType,
) -> Receiver<Vec<HashPair>> {
    let (block_tx, block_rx) = mpsc::channel();

    let mut threads = Vec::new();

    // Block computer threads
    for _ in 0..HASH_GEN_WORKER_THREADS {
        let (thread_tx, thread_rx) = mpsc::channel::<(Sha256, NumSpacesType)>();
        let block_tx_c = block_tx.clone();
        thread::spawn(move || {
            while let Ok(data) = thread_rx.recv() {
                let hashes = get_hashes_for_one_block(data.0, data.1);
                block_tx_c.send(hashes).unwrap();
            }
        });

        threads.push(thread_tx);
    }

    // Work giver thread
    thread::spawn(move || {
        let mut s = Sha256::default();
        s.update(start_str.as_bytes());
        let mut padding_needed =
            SHA_BLOCK_SIZE - (start_str.len() as NumSpacesType) % SHA_BLOCK_SIZE;
        if padding_needed == SHA_BLOCK_SIZE {
            padding_needed = 0;
        }
        s.update(&b" ".repeat(padding_needed as usize));

        let mut curr_thread = 0usize;

        for i in (padding_needed..num_hashes).step_by(SHA_BLOCK_SIZE as usize) {
            threads[curr_thread].send((s.clone(), i)).unwrap();

            s.update(&b" ".repeat(64));

            curr_thread = (curr_thread + 1) % HASH_GEN_WORKER_THREADS;
        }
    });

    block_rx
}
