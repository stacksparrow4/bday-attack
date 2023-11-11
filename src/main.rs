use std::time::Instant;

use std::{
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

use sha::Sha256;

mod sha;

const SHA_BLOCK_SIZE: usize = 64;
const NUM_HASHES: usize = 400 * 1000000;
const THREAD_POOL: usize = 16;
const LAST_HEX_DIGITS_UPPER_BOUND: usize = 16;
const NUM_HASH_BYTES: usize = LAST_HEX_DIGITS_UPPER_BOUND / 2;

fn get_hashes_for_one_block(
    state: Sha256,
    num_spaces: usize,
) -> Vec<([u8; NUM_HASH_BYTES], usize)> {
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

fn get_reversed_hashes(
    start_str: &'static str,
    num_hashes: usize,
    tx: Sender<Vec<([u8; NUM_HASH_BYTES], usize)>>,
) {
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

    // Collate response thread
    thread::spawn(move || {
        let mut v = Vec::with_capacity(num_hashes);

        while let Ok(data) = block_rx.recv() {
            v.extend(data);
        }

        tx.send(v).unwrap();
    });
}

// fn main() {
//     let (real_tx, real_rx) = mpsc::channel();
//     let now = Instant::now();

//     get_reversed_hashes(include_str!("confession_real.txt"), 64, real_tx);

//     let res = real_rx.recv().unwrap();

//     println!("{:?}", res);

//     let elapsed = now.elapsed();
//     println!("Elapsed: {:.2?}", elapsed);
// }

fn main() {
    let (real_tx, real_rx) = mpsc::channel();
    let (fake_tx, fake_rx) = mpsc::channel();

    println!("Starting...");
    let now = Instant::now();

    get_reversed_hashes(include_str!("confession_real.txt"), NUM_HASHES, real_tx);
    get_reversed_hashes(include_str!("confession_fake.txt"), NUM_HASHES, fake_tx);

    let real_v = Arc::new(real_rx.recv().unwrap());
    let mut fake_v = fake_rx.recv().unwrap();

    println!("Got all hashes! (Elapsed: {:.2?})", now.elapsed());
    println!("Sorting...");

    let now = Instant::now();

    fake_v.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let fake_v = Arc::new(fake_v);

    println!(
        "Sorting complete (Elapsed: {:.2?}). Searching for matches...",
        now.elapsed()
    );

    // Binary search for matches
    let now = Instant::now();

    let (tx, rx) = mpsc::channel();

    for thread_num in 0..THREAD_POOL {
        let rv = real_v.clone();
        let fv = fake_v.clone();
        let ltx = tx.clone();
        thread::spawn(move || {
            let mut local_longest_match = 0usize;
            let mut local_longest_real_n = 0usize;
            let mut local_longest_fake_n = 0usize;

            let min_range = (NUM_HASHES / THREAD_POOL) * thread_num;
            let max_range = (NUM_HASHES / THREAD_POOL) * (thread_num + 1);

            for (h, real_n) in rv[min_range..max_range].iter() {
                let mut min = 0;
                let mut max = fv.len();

                while min + 1 < max {
                    let mid = (min + max) / 2;

                    let (mid_h, fake_n) = &fv[mid];

                    match mid_h.partial_cmp(&h).unwrap() {
                        std::cmp::Ordering::Less => {
                            // mid < h
                            min = mid;
                        }
                        std::cmp::Ordering::Equal => {
                            // mid = h
                            println!("Hash collision????! {} {}", real_n, fake_n);
                        }
                        std::cmp::Ordering::Greater => {
                            // mid > h
                            max = mid;
                        }
                    }
                }

                let mut fake_closest = vec![&fv[min]];
                if max != fv.len() {
                    fake_closest.push(&fv[max]);
                }

                for close in fake_closest {
                    let mut i = 0;
                    for (a, b) in h.iter().zip(close.0.iter()) {
                        if a == b {
                            i += 2;
                        } else {
                            if a & 0x0f == b & 0x0f {
                                i += 1;
                            }
                            break;
                        }
                    }

                    if i > local_longest_match {
                        local_longest_match = i;
                        local_longest_real_n = *real_n;
                        local_longest_fake_n = close.1;

                        // println!(
                        //     "Quick update: Found match of length {} using numbers {} and {}",
                        //     local_longest_match, local_longest_real_n, local_longest_fake_n
                        // );
                    }
                }
            }

            ltx.send((
                local_longest_match,
                local_longest_real_n,
                local_longest_fake_n,
            ))
            .unwrap();
        });
    }

    let mut longest_match = 0usize;
    let mut longest_real_n = 0usize;
    let mut longest_fake_n = 0usize;

    for _ in 0..THREAD_POOL {
        let (local_longest_match, local_longest_real_n, local_longest_fake_n) = rx.recv().unwrap();

        if local_longest_match > longest_match {
            longest_match = local_longest_match;
            longest_real_n = local_longest_real_n;
            longest_fake_n = local_longest_fake_n;
        }
    }

    println!("Complete! (Elapsed: {:.2?})", now.elapsed());
    println!(
        "Matched {} hex digits with real {} fake {}",
        longest_match, longest_real_n, longest_fake_n
    );
}
