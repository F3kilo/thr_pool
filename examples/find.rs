use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};
use thr_pool::ThreadPool;

const CHUNK_SIZE: usize = 50_000;

fn main() {
    let data: Arc<[_]> = (0..1_000_000_000).into_iter().rev().collect();
    let to_find = 100_000_000;

    println!("Testing st_find with {}", to_find);
    let found = measure(|| st_find(&data, to_find));
    println!("found: {:?}", found);

    thread::sleep(Duration::from_secs(2));

    println!("------------------",);
    println!("Testing mt_find with {}", to_find);
    let data_clone = Arc::clone(&data);
    let found = measure(|| mt_find(data_clone, to_find));
    println!("found: {:?}", found);

    thread::sleep(Duration::from_secs(2));

    println!("------------------",);
    println!("Testing mt_pool_find with {}", to_find);
    let pool = ThreadPool::new(40).unwrap();
    let data_clone = Arc::clone(&data);
    let found = measure(|| mt_pool_find(data_clone, to_find, pool));
    println!("found: {:?}", found);

    thread::sleep(Duration::from_secs(2));

    println!("------------------",);
    println!("Testing mt_rusty_pool_find with {}", to_find);
    let pool = rusty_pool::ThreadPool::new(40, 40, Duration::from_secs(60));
    let data_clone = Arc::clone(&data);
    let found = measure(|| mt_rusty_pool_find(data_clone, to_find, pool));
    println!("found: {:?}", found);
}

fn measure<T>(f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = f();
    let duration = Instant::now() - start;
    println!("Complete in: {:?}", duration);
    result
}

fn st_find(data: &[i32], val: i32) -> Option<usize> {
    data.iter()
        .enumerate()
        .find(|(_, v)| **v == val)
        .map(|(i, _)| i)
}

fn mt_find(data: Arc<[i32]>, val: i32) -> Option<usize> {
    let chunks_count = data.len() / CHUNK_SIZE;
    let chunks_range = 0..chunks_count;

    let (tx, rx) = mpsc::channel();

    for chunk in chunks_range.clone() {
        let tx = tx.clone();
        let data = data.clone();
        thread::spawn(move || {
            let chunk_start = chunk * CHUNK_SIZE;
            let chunk_end = (chunk + 1) * CHUNK_SIZE;
            let data = &data[chunk_start..chunk_end];

            let found = data
                .iter()
                .enumerate()
                .find(|(_, v)| **v == val)
                .map(|(i, _)| chunk_start + i);

            tx.send(found)
        });
    }

    for _ in chunks_range {
        if let Some(found) = rx.recv().unwrap() {
            return Some(found);
        }
    }

    None
}

fn mt_pool_find(data: Arc<[i32]>, val: i32, pool: ThreadPool) -> Option<usize> {
    let chunks_count = data.len() / CHUNK_SIZE;
    let chunks_range = 0..chunks_count;

    let (tx, rx) = mpsc::channel();

    for chunk in chunks_range.clone() {
        let tx = tx.clone();
        let data = data.clone();
        pool.spawn(move || {
            let chunk_start = chunk * CHUNK_SIZE;
            let chunk_end = (chunk + 1) * CHUNK_SIZE;
            let data = &data[chunk_start..chunk_end];

            let found = data
                .iter()
                .enumerate()
                .find(|(_, v)| **v == val)
                .map(|(i, _)| chunk_start + i);

            let _ = tx.send(found);
        });
    }

    for _ in chunks_range {
        if let Some(found) = rx.recv().unwrap() {
            return Some(found);
        }
    }

    None
}

fn mt_rusty_pool_find(data: Arc<[i32]>, val: i32, pool: rusty_pool::ThreadPool) -> Option<usize> {
    let chunks_count = data.len() / CHUNK_SIZE;
    let chunks_range = 0..chunks_count;

    let (tx, rx) = mpsc::channel();

    for chunk in chunks_range.clone() {
        let tx = tx.clone();
        let data = data.clone();
        pool.execute(move || {
            let chunk_start = chunk * CHUNK_SIZE;
            let chunk_end = (chunk + 1) * CHUNK_SIZE;
            let data = &data[chunk_start..chunk_end];

            let found = data
                .iter()
                .enumerate()
                .find(|(_, v)| **v == val)
                .map(|(i, _)| chunk_start + i);

            let _ = tx.send(found);
        });
    }

    for _ in chunks_range {
        if let Some(found) = rx.recv().unwrap() {
            return Some(found);
        }
    }

    None
}
