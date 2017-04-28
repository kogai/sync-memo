use std::io::prelude::Read;
use std::fs::File;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::PathBuf;

use notify::{Watcher, RecursiveMode, watcher};
use github;

const INTERVAL: u64 = 10;

pub fn watch(path: PathBuf, gist_id: &str) {
    println!("watch: {}", gist_id);
    
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(INTERVAL)).unwrap();
    let path_to_file = path.to_str().unwrap();
    watcher.watch(path_to_file, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(_) => {
                let mut file = File::open(path_to_file).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let gist_modified = github::modify_gist(gist_id,
                                                        path.file_name()
                                                            .unwrap()
                                                            .to_str()
                                                            .unwrap(),
                                                        contents);
                println!("gist modified {:?}", gist_modified);
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

