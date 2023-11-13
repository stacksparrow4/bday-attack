use std::{
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

use crate::{
    constants::{NumSpacesType, CHANNEL_SIZE, HASH_GEN_WORKER_THREADS, SHA_BLOCK_SIZE},
    hash::{Hash, HashBatch, HashPair},
    sha::Sha256,
};
// use libc::sched_param;

fn get_hashes_for_one_block(state: Sha256, num_spaces: NumSpacesType) -> HashBatch {
    let mut results = [HashPair::blank(); SHA_BLOCK_SIZE as usize];

    for i in 0..SHA_BLOCK_SIZE {
        let mut s = state.clone();
        s.update(&b" ".repeat(i as usize));

        let full_hash = s.finish();

        results[i as usize] = HashPair::new(Hash::from_full_hash(full_hash), num_spaces + i);
    }

    results
}

pub(crate) fn get_hashes_in_threads<F>(
    start_str: &'static str,
    num_hashes: NumSpacesType,
    thread_consumers: Vec<F>,
) -> Vec<JoinHandle<()>>
where
    F: FnMut(HashBatch) + Send + 'static,
{
    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();
    let mut threads = Vec::new();

    // Block computer threads
    for mut c in thread_consumers.into_iter() {
        let (thread_tx, thread_rx) = mpsc::sync_channel::<(Sha256, NumSpacesType)>(CHANNEL_SIZE);
        thread_handles.push(thread::spawn(move || {
            // let mut sched_param =
            //     unsafe { std::mem::MaybeUninit::<libc::sched_param>::zeroed().assume_init() };
            // sched_param.sched_priority = -20;
            // unsafe {
            //     libc::pthread_setschedparam(libc::pthread_self(), libc::SCHED_RR, &sched_param)
            // };

            while let Ok(data) = thread_rx.recv() {
                let hashes = get_hashes_for_one_block(data.0, data.1);
                c(hashes);
            }
        }));

        threads.push(thread_tx);
    }

    // Work giver thread
    thread::spawn(move || {
        // let mut sched_param =
        //     unsafe { std::mem::MaybeUninit::<libc::sched_param>::zeroed().assume_init() };
        // sched_param.sched_priority = -20;
        // unsafe { libc::pthread_setschedparam(libc::pthread_self(), libc::SCHED_RR, &sched_param) };

        let mut s = Sha256::default();
        s.update(start_str.as_bytes());
        let mut padding_needed =
            SHA_BLOCK_SIZE - (start_str.len() as NumSpacesType) % SHA_BLOCK_SIZE;
        if padding_needed == SHA_BLOCK_SIZE {
            padding_needed = 0;
        }
        s.update(&b" ".repeat(padding_needed as usize));

        let mut curr_thread = 0usize;

        for i in (padding_needed..num_hashes).step_by(SHA_BLOCK_SIZE as usize) {
            threads[curr_thread].send((s.clone(), i)).unwrap();

            s.update(&b" ".repeat(64));

            curr_thread = (curr_thread + 1) % HASH_GEN_WORKER_THREADS;
        }
    });

    thread_handles
}

pub(crate) fn get_reversed_hashes(
    start_str: &'static str,
    num_hashes: NumSpacesType,
) -> Receiver<HashBatch> {
    let (block_tx, block_rx) = mpsc::sync_channel(CHANNEL_SIZE);

    get_hashes_in_threads(
        start_str,
        num_hashes,
        (0..HASH_GEN_WORKER_THREADS)
            .map(|_| {
                let block_tx_c = block_tx.clone();

                move |hashes| {
                    block_tx_c.send(hashes).unwrap();
                }
            })
            .collect(),
    );

    block_rx
}
