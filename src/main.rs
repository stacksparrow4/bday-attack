use clap::{command, Command};
use disk_hash_map::DiskHashMapReader;
use std::fs;
use std::io::ErrorKind;
use std::time::Instant;

mod constants;
mod disk_hash_map;
mod hash;
mod hash_gen;
mod sha;

use crate::constants::{HASH_TABLE_FILE_QUOTA, NUM_HASHES};
use crate::disk_hash_map::DiskHashMapWriter;
use crate::hash_gen::get_reversed_hashes;

fn gen_table() {
    println!(
        "Allocating {:.3?}gb file for hash map",
        (HASH_TABLE_FILE_QUOTA as f32) / 1024.0 / 1024.0 / 1024.0
    );

    match fs::remove_file("fake.tmp") {
        Ok(_) => {}
        Err(e) => {
            if e.kind() != ErrorKind::NotFound {
                panic!("{}", e);
            }
        }
    }

    println!("Generating hash table...");
    let now = Instant::now();

    let hashes_fake = get_reversed_hashes(include_str!("confession_fake.txt"), NUM_HASHES);

    let mut hash_map = DiskHashMapWriter::new("fake.tmp");

    while let Ok(fake_hashes) = hashes_fake.recv() {
        for fake_hash in fake_hashes {
            hash_map.insert_pair(fake_hash);
        }
    }

    hash_map.flush();

    println!("Finished generating hash table in {:.2?}", now.elapsed());
}

fn search() {
    println!("Searching hash table for collisions...");
    let now = Instant::now();

    let hashes_real = get_reversed_hashes(include_str!("confession_real.txt"), NUM_HASHES);

    let mut reader = DiskHashMapReader::new("fake.tmp");

    while let Ok(real_hashes) = hashes_real.recv() {
        for real_hash in real_hashes {
            if let Some(matched) = reader.search(real_hash.hash) {
                println!(
                    "Collision found with real {} fake {}",
                    real_hash.num_spaces, matched.num_spaces
                );
            }
        }
    }

    println!("Finished search in {:.2?}", now.elapsed());
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
