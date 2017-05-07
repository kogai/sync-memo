use std::os::unix::net::{UnixListener, UnixStream};
use std::fs;
use std::io::{Write, Read};
use std::process::exit;
use std::path::PathBuf;
use std::thread::spawn;

use serde_json;

use client::{Command, Response};
use handler::FileHandler;
use github::get_gist;

pub const SOCKET_ADDR: &'static str = "/tmp/sync-memo.sock";
pub const PID_FILE: &'static str = "/tmp/sync-memo.pid";

#[derive(Debug)]
pub struct Daemon {
    listener: UnixListener,
    file_handler: FileHandler,
}

impl Daemon {
    pub fn new(path: PathBuf) -> Self {
        fs::remove_file(SOCKET_ADDR).unwrap_or(());
        Daemon {
            listener: UnixListener::bind(SOCKET_ADDR).expect("daemon start failed"),
            file_handler: FileHandler::new(path),
        }
    }

    pub fn listen(&self) {
        info!("Waiting for connection from client...");
        self.file_handler.watch_all_files();

        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    use client::Command::*;
                    let command = extract_command(&mut stream);
                    match command {
                        Add(file_names) => {
                            for file_name in file_names {
                                let add_file = self.file_handler.add_file(file_name);
                                self.file_handler.watch_file(add_file);
                            }
                        }
                        Show => {
                            let file_ids = self.file_handler.get_file_ids();
                            let gists = file_ids.into_iter()
                                .map(|id| {
                                    spawn(move || get_gist(&id))
                                })
                                .collect::<Vec<_>>();

                            for gist in gists {
                                let gist = gist.join().expect("something wrong with thread");
                                info!("{}", gist);
                            }
                        }
                        Kill => {
                            let response = Response::with_info("daemon killed");
                            stream.write_all(response.to_string().as_bytes()).expect("write in daemon");
                            exit(1);
                        }
                    };
                }
                Err(e) => error!("{:?}", e),
            }
        }
    }
}

fn extract_command(stream: &mut UnixStream) -> Command {
    let mut buffer = [0; 1000];
    loop {
        match stream.read(&mut buffer) {
            Ok(chunk_size) => {
                println!("chunk size -> {}", chunk_size);
                break;
            }
            Err(error) => {
                error!("read line failed... {:?}", error);
                break;
            }
        }
    }

    let filtered = buffer.to_vec().into_iter().filter(|x| *x > 0).collect::<Vec<_>>();
    let result = String::from_utf8(filtered).unwrap();

    serde_json::from_str(&result).expect("parse failed")
}
