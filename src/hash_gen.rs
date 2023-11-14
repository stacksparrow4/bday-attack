use std::{
    fs,
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

use crate::{
    constants::{LineMaskType, CHANNEL_SIZE, NUM_THREADS, THREADED_BITS},
    hash::{HashLastDigits, HashLastDigitsPair},
    sha::Sha256,
};

fn hash_lines(data: &str, mut line_mask: LineMaskType) -> HashLastDigits {
    let mut s = Sha256::default();
    for l in data.lines() {
        s.update(l.as_bytes());
        if line_mask & 1 == 1 {
            s.update(b" ");
        }
        s.update(b"\n");
        line_mask >>= 1;
    }
    HashLastDigits::from_full_hash(s.finish())
}

pub(crate) fn line_mask_to_file(fname: &str, data: &str, mut line_mask: LineMaskType) {
    // fs::write(fname,
    let mut s = String::new();
    for l in data.lines() {
        s.push_str(l);
        if line_mask & 1 == 1 {
            s.push(' ');
        }
        s.push('\n');
        line_mask >>= 1;
    }
    fs::write(fname, s).unwrap();
}

pub(crate) fn get_hashes_in_threads<F, G>(
    start_str: &'static str,
    num_hashes: LineMaskType,
    consumer_generator: F,
) -> Vec<JoinHandle<()>>
where
    F: Fn(LineMaskType) -> G,
    G: FnMut(HashLastDigitsPair) + Send + 'static,
{
    (0..NUM_THREADS)
        .map(|worker_id| {
            let mut consumer = consumer_generator(worker_id);

            thread::spawn(move || {
                for i in 0..(num_hashes / NUM_THREADS) {
                    let line_mask = worker_id | (i << THREADED_BITS);
                    let result = hash_lines(start_str, line_mask);
                    consumer((result, line_mask));
                }
            })
        })
        .collect()
}

pub(crate) fn get_hashes(
    start_str: &'static str,
    num_hashes: LineMaskType,
) -> Receiver<HashLastDigitsPair> {
    let (block_tx, block_rx) = mpsc::sync_channel(CHANNEL_SIZE);

    get_hashes_in_threads(start_str, num_hashes, |_| {
        let block_tx_c = block_tx.clone();

        move |hashes| {
            block_tx_c.send(hashes).unwrap();
        }
    });

    block_rx
}
