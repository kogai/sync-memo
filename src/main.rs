#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;
#[macro_use] extern crate log;

extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate config_file_handler;
extern crate hyper;
extern crate notify;
extern crate log4rs;

mod github;
mod watcher;
mod handler;
mod daemon;
mod client;

use std::env;
use clap::{App, Arg, SubCommand};

fn main() {
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();
    
    let path = env::home_dir()
        .and_then(|x| Some(x.join("sync-memo").join(".sync-memo-config.json")))
        .expect("setting file missing");

    let matches = App::new("sync-memo")
        .version("0.1.0")
        .about("sync local memo files via gist")
        .subcommand(SubCommand::with_name("add")
            .about("add file to sync")
            .arg(Arg::with_name("files")
                .required(true)
                .multiple(true)))
        .subcommand(SubCommand::with_name("watch").about("start watcher daemon"))
        .subcommand(SubCommand::with_name("show").about("show all memo"))
        .subcommand(SubCommand::with_name("kill").about("kill watcher daemon"))
        // TODO: add [remove] command
        .get_matches();

    let c = client::Client::new();

    match matches.subcommand() {
        ("watch", Some(_)) => {
            let server = daemon::Daemon::new(path);
            server.listen();
        }
        ("add", Some(sub_matches)) => {
            let file_names = values_t!(sub_matches.values_of("files"), String)
                .expect("path to files missing");
            c.send(client::Command::Add(file_names))
        }
        ("show", Some(_)) => c.send(client::Command::Show), 
        ("kill", Some(_)) => c.send(client::Command::Kill),
        _ => {}
    };
}
