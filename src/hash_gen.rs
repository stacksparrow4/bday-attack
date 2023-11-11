use std::{
    sync::mpsc::{self, Receiver},
    thread,
};

use crate::{
    constants::{NUM_HASH_BYTES, SHA_BLOCK_SIZE, THREAD_POOL},
    sha::Sha256,
};

pub type HashPair = ([u8; NUM_HASH_BYTES], usize);

fn get_hashes_for_one_block(state: Sha256, num_spaces: usize) -> Vec<HashPair> {
    (0..SHA_BLOCK_SIZE)
        .map(|i| {
            let mut s = state.clone();
            s.update(&b" ".repeat(i));

            let full_hash = s.finish();

            (
                [
                    full_hash[31],
                    full_hash[30],
                    full_hash[29],
                    full_hash[28],
                    full_hash[27],
                    full_hash[26],
                    full_hash[25],
                    full_hash[24],
                ],
                num_spaces + i,
            )
        })
        .collect()
}

fn get_reversed_hashes(start_str: &'static str, num_hashes: usize) -> Receiver<Vec<HashPair>> {
    let (block_tx, block_rx) = mpsc::channel();

    let mut threads = Vec::new();

    // Block computer threads
    for _ in 0..THREAD_POOL {
        let (thread_tx, thread_rx) = mpsc::channel::<(Sha256, usize)>();
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
        let mut padding_needed = SHA_BLOCK_SIZE - start_str.len() % SHA_BLOCK_SIZE;
        if padding_needed == SHA_BLOCK_SIZE {
            padding_needed = 0;
        }
        s.update(&b" ".repeat(padding_needed));

        let mut curr_thread = 0usize;

        for i in (padding_needed..num_hashes).step_by(SHA_BLOCK_SIZE) {
            threads[curr_thread].send((s.clone(), i)).unwrap();

            s.update(&b" ".repeat(64));

            curr_thread = (curr_thread + 1) % THREAD_POOL;
        }
    });

    block_rx
}
