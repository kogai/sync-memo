#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate config_file_handler;
extern crate hyper;
extern crate notify;

mod github;
mod watcher;

use std::env;

fn main() {
    let path = env::home_dir().and_then(|x| Some(x.join("sync-memo").join("memo.md"))).unwrap();
    watcher::watch(path, "0c410ee3dcdb1d709c281eb40c2e8396");
    // let gists = github::get_gist("5c48d55cac77922fb1dd2162e48256f7");
    // let gist_created = github::create_gist();
    // let gist_modified = github::modify_gist("0c410ee3dcdb1d709c281eb40c2e8396");
} 