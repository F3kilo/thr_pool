use std::fs::{self, File};
use std::sync::Barrier;
use std::{io, thread};

const FILES_PATH: &str = "./target/my_files";
const COUNT: usize = 5;

fn main() {
    let barrier = Barrier::new(COUNT);
    let barrier_ref = &barrier;

    thread::scope(move |scope| {
        for i in 0..COUNT as u8 {
            scope.spawn(move || {
                create_file(i);
                barrier_ref.wait();
                check_files();
            });
        }
    });

    remove_files();
}

fn create_file(i: u8) {
    fs::create_dir_all(FILES_PATH).unwrap();
    File::create(format!("{FILES_PATH}/{i}.txt")).unwrap();
}

fn check_files() {
    let read_dir = fs::read_dir(FILES_PATH).unwrap();
    let thread_id = std::thread::current().id();
    let _lock = io::stdout().lock();
    println!("Found for {thread_id:?}:");
    for entry in read_dir {
        println!("\t{}", entry.unwrap().file_name().to_str().unwrap())
    }
}

fn remove_files() {
    fs::remove_dir_all(FILES_PATH).unwrap();
}
