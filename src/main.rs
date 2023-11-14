use constants::NumSpacesType;
use hash::HashLastDigits;
use rustc_hash::FxHashSet;
use std::hash::BuildHasherDefault;
use std::sync::Arc;
use std::time::Instant;

mod constants;
mod hash;
mod hash_gen;
mod progress_updater;
mod sha;

use crate::constants::{DESIRED_HEX_MATCHES, HASH_SEARCH_WORKER_THREADS, NUM_HASHES};
use crate::hash::HashLastDigitsPair;
use crate::hash_gen::{get_hashes_in_threads, get_reversed_hashes};
use crate::progress_updater::Progress;

type TableType = FxHashSet<HashLastDigits>;

fn gen_table() -> TableType {
    println!("Generating hash table...");
    let now = Instant::now();

    let mut prog = Progress::new(NUM_HASHES as usize);

    let hashes_fake = get_reversed_hashes(include_str!("confession_fake.txt"), NUM_HASHES);

    let mut table =
        FxHashSet::with_capacity_and_hasher(NUM_HASHES as usize, BuildHasherDefault::default());

    while let Ok(fake_hashes) = hashes_fake.recv() {
        for fake_hash in fake_hashes {
            table.insert(fake_hash.0);
            prog.increment();
        }
    }

    println!("\nFinished generating hash table in {:.2?}", now.elapsed());

    table
}

fn search_specific(h: &HashLastDigits) -> Option<NumSpacesType> {
    let fakes_c = get_reversed_hashes(include_str!("confession_fake.txt"), NUM_HASHES);

    while let Ok(fakes) = fakes_c.recv() {
        for fake in fakes {
            if fake.0 == *h {
                return Some(fake.1);
            }
        }
    }

    return None;
}

fn search(table: TableType) {
    println!("Searching hash table for collisions...");
    let now = Instant::now();

    let table = Arc::new(table);

    let handles = get_hashes_in_threads(
        include_str!("confession_real.txt"),
        NUM_HASHES,
        (0..HASH_SEARCH_WORKER_THREADS)
            .map(|worker_id| {
                let table = table.clone();

                let mut prog = Progress::new((NUM_HASHES as usize) / HASH_SEARCH_WORKER_THREADS);

                move |real_hashes: Vec<HashLastDigitsPair>| {
                    for real_hash in real_hashes {
                        if table.contains(&real_hash.0) {
                            println!(
                                "\nCollision found with real {}. Computing match for fake...\n",
                                real_hash.1
                            );

                            println!(
                                "\nCOLLISION OF LENGTH {} REAL {} FAKE {}\n",
                                DESIRED_HEX_MATCHES,
                                real_hash.1,
                                search_specific(&real_hash.0).unwrap()
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
    let t = gen_table();
    search(t);
}
