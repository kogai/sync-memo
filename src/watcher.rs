use std::io::prelude::Read;
use std::fs::File;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use github;

const INTERVAL: u64 = 10;

// TODO: move to handler mod
pub fn watch(path: String,
             gist_id: &str,
             (tx, rx): (Sender<DebouncedEvent>, Receiver<DebouncedEvent>)) {
    info!("watching file {}", &path);

    let mut watcher = watcher(tx, Duration::from_secs(INTERVAL)).unwrap();
    let path_to_file = path.as_str();
    watcher.watch(path_to_file, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(_) => {
                let mut file = File::open(path_to_file).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let gist_modified = github::modify_gist(gist_id, path.clone(), contents);
                info!("gist modified {:?}", gist_modified);
            }
            Err(e) => {
                error!("watch error: {:?}", e);
                watcher.unwatch(&path).unwrap();
                break;
            }
        }
    }
}
