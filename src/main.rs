#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate config_file_handler;
extern crate hyper;
extern crate notify;
extern crate crossbeam;

mod github;
mod watcher;
mod handler;

use std::env;
use clap::{App, Arg, SubCommand};

fn main() {
    let path = env::home_dir().and_then(|x| Some(x.join("sync-memo").join(".sync-memo-config.json"))).unwrap();
    let file_handler = handler::FileHandler::new(path);

    let matches = App::new("syncm")
        .version("0.0.1")
        .about("sync local memo files via gist")
        .subcommand(
            SubCommand::with_name("add")
            .about("add file to sync")
            .arg(
                Arg::with_name("files")
                .required(true)
                .multiple(true)
            )
        )
        .subcommand(
            SubCommand::with_name("watch")
            .about("watch files to sync")
        )
        .get_matches();
    
    if let Some(m) = matches.subcommand_matches("add") {
        let file_names = values_t!(m.values_of("files"), String).unwrap();
        for file_name in file_names {
            file_handler.add_files(file_name);
        }
    }
    
//     let handlers = file_handler.watch_all_files();
//    println!("watch start..."); 
//     for h in handlers {
//         println!("watch thread deliminated with {:?}", h.join());
//     };

    // let gists = github::get_gist("5c48d55cac77922fb1dd2162e48256f7");
    // let gist_created = github::create_gist();
} 