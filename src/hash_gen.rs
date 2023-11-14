use std::{
    fs,
    sync::{
        mpsc::{self, Receiver},
        Arc,
    },
    thread::{self, JoinHandle},
};

use bitvec::vec::BitVec;

use crate::{
    constants::{LineMaskType, CHANNEL_SIZE, NUM_HASHES, NUM_THREADS},
    hash::{HashLastDigits, HashLastDigitsPair},
    sha::Sha256,
};

fn hash_lines(
    lines: &Vec<&str>,
    mut line_mask: LineMaskType,
    // cache: Vec<Sha256>,
    // prev_mask: LineMaskType,
) -> HashLastDigits {
    let num_lines = lines.len();
    let shift = (num_lines - 1) * 2;

    let mut s = Sha256::default();
    for l in lines {
        s.update(l.as_bytes());

        let curr = (line_mask >> shift) & 0b11;
        match curr {
            0b01 => s.update(b" "),
            0b10 => s.update(b"\t"),
            0b11 => s.update(&[0xc2, 0xa0]), // No break space
            _ => {}
        }

        s.update(b"\n");
        line_mask <<= 2;
    }

    HashLastDigits::from_full_hash(s.finish())
}

pub(crate) fn line_mask_to_file(fname: &str, data: &str, mut line_mask: LineMaskType) {
    let num_lines = data.lines().count();
    let shift = (num_lines - 1) * 2;

    let mut s = String::new();
    for l in data.lines() {
        s.push_str(l);

        let curr = (line_mask >> shift) & 0b11;
        match curr {
            0b01 => s.push(' '),
            0b10 => s.push('\t'),
            0b11 => s.push('\u{00A0}'), // No break space
            _ => {}
        }

        s.push('\n');
        line_mask <<= 2;
    }
    fs::write(fname, s).unwrap();
}

pub(crate) fn get_hashes_in_threads<F, G>(
    start_str: &'static str,
    consumer_generator: F,
    mask: Option<BitVec>,
) -> Vec<JoinHandle<()>>
where
    F: Fn(LineMaskType) -> G,
    G: FnMut(HashLastDigitsPair) + Send + 'static,
{
    let mask = mask.map(Arc::new);

    (0..NUM_THREADS)
        .map(|worker_id| {
            let mut consumer = consumer_generator(worker_id);

            let min = (worker_id * NUM_HASHES) / NUM_THREADS;
            let max = ((worker_id + 1) * NUM_HASHES) / NUM_THREADS;

            let lines: Vec<&str> = start_str.lines().collect();

            let mask = mask.clone();

            thread::spawn(move || {
                for line_mask in min..max {
                    if mask.is_none() || *mask.as_ref().unwrap().get(line_mask).unwrap() {
                        let result = hash_lines(&lines, line_mask);
                        consumer((result, line_mask));
                    }
                }
            })
        })
        .collect()
}

pub(crate) fn get_hashes(
    start_str: &'static str,
    mask: Option<BitVec>,
) -> Receiver<HashLastDigitsPair> {
    let (block_tx, block_rx) = mpsc::sync_channel(CHANNEL_SIZE);

    get_hashes_in_threads(
        start_str,
        |_| {
            let block_tx_c = block_tx.clone();

            move |hashes| {
                block_tx_c.send(hashes).unwrap();
            }
        },
        mask,
    );

    block_rx
}
