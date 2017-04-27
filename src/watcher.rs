use std::io::prelude::Read;
use std::fs::File;
use std::sync::mpsc::channel;
use notify::{Watcher, RecursiveMode, watcher};
use std::time::Duration;

const INTERVAL: u64 = 10;

pub fn watch(path: &str) {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(INTERVAL)).unwrap();

    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(_) => {
                let mut file = File::open(path).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                println!("{:?}", contents);
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

