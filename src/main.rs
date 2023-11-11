use clap::{command, Command};
use std::time::Instant;

mod constants;
mod disk_hash_map;
mod hash;
mod hash_gen;
mod sha;

use crate::constants::{HASH_TABLE_FILE_QUOTA, NUM_HASHES};
use crate::disk_hash_map::DiskHashMapWriter;
use crate::hash_gen::get_reversed_hashes;

fn check_hash_table_density() {
    println!(
        "Allocating {:.2?} gb file for hash map",
        (HASH_TABLE_FILE_QUOTA as f32) / 1024.0 / 1024.0 / 1024.0
    );

    println!("We are generating {} hashes", NUM_HASHES);
    println!(
        "The table allows {} entries",
        DiskHashMapWriter::num_entries()
    );
    let hash_table_density = (NUM_HASHES as f32) / (DiskHashMapWriter::num_entries() as f32);
    println!(
        "This means that {:.2?}% of the table should be used.",
        100.0 * hash_table_density
    );

    if hash_table_density > 0.5 {
        panic!("Density too large! Aborting...");
    }
}

fn gen_table() {
    check_hash_table_density();

    println!("Generating hash table...");
    let now = Instant::now();

    let hashes_fake = get_reversed_hashes(include_str!("confession_fake.txt"), NUM_HASHES);

    let mut hash_map = DiskHashMapWriter::new("fake.tmp");

    while let Ok(fake_hashes) = hashes_fake.recv() {
        for fake_hash in fake_hashes {
            hash_map.insert_pair(fake_hash);
        }
    }

    println!("Finished generating hash table in {:.2?}", now.elapsed());
}

fn search() {
    //TODO
}

fn main() {
    let matches = command!()
        .subcommand(Command::new("gentable").about("Generate the hash table to disk"))
        .subcommand(Command::new("search").about("Search the generated hash table for matches"))
        .subcommand(Command::new("all").about("Equivalent to gentable + search"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("gentable") {
        gen_table();
    } else if let Some(_) = matches.subcommand_matches("search") {
        search();
    } else if let Some(_) = matches.subcommand_matches("all") {
        gen_table();
        search();
    } else {
        println!("Run with --help for help");
    }
}
