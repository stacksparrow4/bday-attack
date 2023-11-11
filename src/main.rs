use std::time::Instant;

use std::{
    sync::{
        mpsc::{self},
        Arc,
    },
    thread,
};

mod constants;
mod hash_disk_array;
mod hash_gen;
mod sha;

use hash_disk_array::HashDiskArray;
use sha::Sha256;

fn main() {
    let mut hda = HashDiskArray::new("tmp.dat");

    hda.add_hash(([0, 1, 2, 3, 4, 5, 6, 7], 0));
    hda.add_hash(([0, 1, 2, 3, 4, 5, 6, 7], 1));
    hda.add_hash(([0, 1, 2, 3, 4, 5, 6, 7], 2));
    hda.add_hash(([0, 1, 2, 3, 44, 5, 6, 7], 3));

    for p in hda.into_iter() {
        println!("{:x?}, {}", p.0, p.1);
    }
}

// fn main() {
//     println!("Starting...");
//     let now = Instant::now();

//     get_reversed_hashes(include_str!("confession_real.txt"), NUM_HASHES);
//     get_reversed_hashes(include_str!("confession_fake.txt"), NUM_HASHES);

//     let real_v = Arc::new(real_rx.recv().unwrap());
//     let mut fake_v = fake_rx.recv().unwrap();

//     println!("Got all hashes! (Elapsed: {:.2?})", now.elapsed());
//     println!("Sorting...");

//     let now = Instant::now();

//     fake_v.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

//     let fake_v = Arc::new(fake_v);

//     println!(
//         "Sorting complete (Elapsed: {:.2?}). Searching for matches...",
//         now.elapsed()
//     );

//     // Binary search for matches
//     let now = Instant::now();

//     let (tx, rx) = mpsc::channel();

//     for thread_num in 0..THREAD_POOL {
//         let rv = real_v.clone();
//         let fv = fake_v.clone();
//         let ltx = tx.clone();
//         thread::spawn(move || {
//             let mut local_longest_match = 0usize;
//             let mut local_longest_real_n = 0usize;
//             let mut local_longest_fake_n = 0usize;

//             let min_range = (NUM_HASHES / THREAD_POOL) * thread_num;
//             let max_range = (NUM_HASHES / THREAD_POOL) * (thread_num + 1);

//             for (h, real_n) in rv[min_range..max_range].iter() {
//                 let mut min = 0;
//                 let mut max = fv.len();

//                 while min + 1 < max {
//                     let mid = (min + max) / 2;

//                     let (mid_h, fake_n) = &fv[mid];

//                     match mid_h.partial_cmp(&h).unwrap() {
//                         std::cmp::Ordering::Less => {
//                             // mid < h
//                             min = mid;
//                         }
//                         std::cmp::Ordering::Equal => {
//                             // mid = h
//                             println!("Hash collision????! {} {}", real_n, fake_n);
//                         }
//                         std::cmp::Ordering::Greater => {
//                             // mid > h
//                             max = mid;
//                         }
//                     }
//                 }

//                 let mut fake_closest = vec![&fv[min]];
//                 if max != fv.len() {
//                     fake_closest.push(&fv[max]);
//                 }

//                 for close in fake_closest {
//                     let mut i = 0;
//                     for (a, b) in h.iter().zip(close.0.iter()) {
//                         if a == b {
//                             i += 2;
//                         } else {
//                             if a & 0x0f == b & 0x0f {
//                                 i += 1;
//                             }
//                             break;
//                         }
//                     }

//                     if i > local_longest_match {
//                         local_longest_match = i;
//                         local_longest_real_n = *real_n;
//                         local_longest_fake_n = close.1;

//                         // println!(
//                         //     "Quick update: Found match of length {} using numbers {} and {}",
//                         //     local_longest_match, local_longest_real_n, local_longest_fake_n
//                         // );
//                     }
//                 }
//             }

//             ltx.send((
//                 local_longest_match,
//                 local_longest_real_n,
//                 local_longest_fake_n,
//             ))
//             .unwrap();
//         });
//     }

//     let mut longest_match = 0usize;
//     let mut longest_real_n = 0usize;
//     let mut longest_fake_n = 0usize;

//     for _ in 0..THREAD_POOL {
//         let (local_longest_match, local_longest_real_n, local_longest_fake_n) = rx.recv().unwrap();

//         if local_longest_match > longest_match {
//             longest_match = local_longest_match;
//             longest_real_n = local_longest_real_n;
//             longest_fake_n = local_longest_fake_n;
//         }
//     }

//     println!("Complete! (Elapsed: {:.2?})", now.elapsed());
//     println!(
//         "Matched {} hex digits with real {} fake {}",
//         longest_match, longest_real_n, longest_fake_n
//     );
// }
