use constants::NumSpacesType;
use hash::Hash;
// use std::collections::HashMap;
use rustc_hash::FxHashMap;
use std::sync::Arc;
use std::time::Instant;

mod constants;
mod hash;
mod hash_gen;
mod progress_updater;
mod sha;

use crate::constants::{HASH_SEARCH_WORKER_THREADS, NUM_HASHES};
use crate::hash::HashPair;
use crate::hash_gen::{get_hashes_in_threads, get_reversed_hashes};
use crate::progress_updater::Progress;
use std::hash::BuildHasherDefault;

fn gen_table() -> FxHashMap<Hash, NumSpacesType> {
    println!("Generating hash table...");
    let now = Instant::now();

    let mut prog = Progress::new(NUM_HASHES as usize);

    let hashes_fake = get_reversed_hashes(include_str!("confession_fake.txt"), NUM_HASHES);

    let mut hash_map =
        FxHashMap::with_capacity_and_hasher(NUM_HASHES as usize, BuildHasherDefault::default());

    while let Ok(fake_hashes) = hashes_fake.recv() {
        for fake_hash in fake_hashes {
            hash_map.insert(fake_hash.hash, fake_hash.num_spaces);
            prog.increment();
        }
    }

    println!("Finished generating hash table in {:.2?}", now.elapsed());

    hash_map
}

fn search(hash_map: FxHashMap<Hash, NumSpacesType>) {
    println!("Searching hash table for collisions...");
    let now = Instant::now();

    let hash_map = Arc::new(hash_map);

    let handles = get_hashes_in_threads(
        include_str!("confession_real.txt"),
        NUM_HASHES,
        (0..HASH_SEARCH_WORKER_THREADS)
            .map(|worker_id| {
                let hash_map = hash_map.clone();

                let mut prog = Progress::new((NUM_HASHES as usize) / HASH_SEARCH_WORKER_THREADS);

                move |real_hashes: Vec<HashPair>| {
                    for real_hash in real_hashes {
                        if let Some(matched) = hash_map.get(&real_hash.hash) {
                            println!(
                                "Collision found with real {} fake {}",
                                real_hash.num_spaces, matched
                            );
                            std::process::exit(0);
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

    println!("Finished search in {:.2?}", now.elapsed());
}

fn main() {
    println!();
    let hm = gen_table();
    search(hm);
}
