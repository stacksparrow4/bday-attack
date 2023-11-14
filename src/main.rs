use bitvec::bitvec;
use bitvec::vec::BitVec;
use constants::NumSpacesType;
use hash::HashLastDigits;
use rustc_hash::FxHashMap;
use std::sync::Arc;
use std::time::Instant;

mod constants;
mod hash;
mod hash_gen;
mod progress_updater;
mod sha;

use crate::constants::{HASH_SEARCH_WORKER_THREADS, NUM_HASHES, PREHASH_SIZE};
use crate::hash::HashLastDigitsPair;
use crate::hash_gen::{get_hashes, get_hashes_in_threads};
use crate::progress_updater::Progress;

const FAKE: &str = include_str!("confession_fake.txt");
const REAL: &str = include_str!("confession_real.txt");

fn gen_fake_filter() -> (BitVec, usize) {
    println!("Generating filter...");
    let now = Instant::now();

    let mut prog = Progress::new((NUM_HASHES as usize) * 2);

    let mut all_real = bitvec![0; 1<<PREHASH_SIZE];

    let rx = get_hashes(REAL, NUM_HASHES);
    while let Ok(hs) = rx.recv() {
        for (h, _) in hs {
            all_real.set(h.prehash(), true);
            prog.increment();
        }
    }

    let mut filter_fake = bitvec![0; NUM_HASHES as usize + 64];

    let mut tot = 0usize;

    let rx = get_hashes(FAKE, NUM_HASHES);
    while let Ok(hs) = rx.recv() {
        for (h, hn) in hs {
            let h32 = h.prehash();

            if *all_real.get(h32).unwrap() {
                // This is a collision with something in the real array
                filter_fake.set(hn as usize, true);

                tot += 1;
            }

            prog.increment();
        }
    }

    println!("\nRatio: {}%", 100 * tot / (NUM_HASHES as usize));

    println!("\nFinished generating filter in {:.2?}", now.elapsed());

    (filter_fake, tot)
}

fn gen_table(filter_fake: BitVec, total: usize) -> FxHashMap<HashLastDigits, NumSpacesType> {
    println!("Generating hash table...");
    let now = Instant::now();

    let mut prog = Progress::new(total);

    let hashes_fake = get_hashes(FAKE, NUM_HASHES);

    let mut hash_map = FxHashMap::default();

    while let Ok(fake_hashes) = hashes_fake.recv() {
        for (fake_hash, fake_num) in fake_hashes {
            if *filter_fake.get(fake_num as usize).unwrap() {
                hash_map.insert(fake_hash, fake_num);
                prog.increment();
            }
        }
    }

    println!("\nFinished generating hash table in {:.2?}", now.elapsed());

    hash_map
}

fn search(hash_map: FxHashMap<HashLastDigits, NumSpacesType>) {
    println!("Searching hash table for collisions...");
    let now = Instant::now();

    let hash_map = Arc::new(hash_map);

    let handles = get_hashes_in_threads(
        REAL,
        NUM_HASHES,
        (0..HASH_SEARCH_WORKER_THREADS)
            .map(|worker_id| {
                let hash_map = hash_map.clone();

                let mut prog = Progress::new((NUM_HASHES as usize) / HASH_SEARCH_WORKER_THREADS);

                move |real_hashes: Vec<HashLastDigitsPair>| {
                    for real_hash in real_hashes {
                        if let Some(matched) = hash_map.get(&real_hash.0) {
                            println!(
                                "\nCollision found with real {} fake {}\n",
                                real_hash.1, matched
                            );
                        }

                        if worker_id == 0 {
                            prog.increment();
                        }
                    }
                }
            })
            .collect(),
    );

    for h in handles {
        h.join().unwrap();
    }

    println!("\nFinished search in {:.2?}", now.elapsed());
}

fn main() {
    println!();
    let (filter_fake, tot) = gen_fake_filter();
    let hm = gen_table(filter_fake, tot);
    search(hm);
}
