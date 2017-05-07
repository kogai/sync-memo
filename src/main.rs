#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate config_file_handler;
extern crate hyper;
extern crate notify;
extern crate log4rs;
extern crate daemonize;

mod github;
mod watcher;
mod handler;
mod daemon;
mod client;

use std::env;
use clap::{App, Arg, SubCommand};
use daemonize::Daemonize;

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
        // TODO: add [status] command
        .get_matches();

    let c = client::Client::new();

    match matches.subcommand() {
        ("watch", Some(_)) => {
            let daemonize = Daemonize::new()
                .pid_file(daemon::PID_FILE)
                .working_directory("/tmp")
                .privileged_action(|| "Executed before drop privileges");

            let server = daemon::Daemon::new(path);
            let response = server.get_watch_files();
            response.write_log();
            
            match daemonize.start() {
                Ok(_) => {
                    info!("daemonized success");
                    server.listen();
                }
                Err(error) => error!("{}", error),
            }
        }
        ("add", Some(sub_matches)) => {
            let file_names = values_t!(sub_matches.values_of("files"), String)
                .expect("path to files missing");
            let response = c.send(client::Command::Add(file_names));
            response.write_log();
        }
        ("show", Some(_)) => {
            let response = c.send(client::Command::Show);
            response.write_log();
        }, 
        ("kill", Some(_)) => {
            let response = c.send(client::Command::Kill);
            response.write_log();
        },
        _ => {}
    };
}
