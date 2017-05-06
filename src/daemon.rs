use std::os::unix::net::{UnixListener, UnixStream};
use std::fs;
use std::io::{BufReader, BufRead};
use std::process::exit;
use std::path::PathBuf;
use serde_json;

use client::Command;
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
                Ok(stream) => {
                    use client::Command::*;
                    let command = extract_command(&stream);
                    match command {
                        Add(file_names) => {
                            for file_name in file_names {
                                let add_file = self.file_handler.add_file(file_name);
                                self.file_handler.watch_file(add_file);
                            }
                        }
                        Show => {
                            let file_ids = self.file_handler.get_file_ids();
                            // TODO: speed up with concurrent request
                            let gists = file_ids.into_iter()
                                .map(|id| get_gist(&id))
                                .collect::<Vec<_>>();
                            for gist in &gists {
                                info!("{}", gist);
                            }
                        }
                        Kill => {
                            info!("daemon killed");
                            exit(1);
                        }
                    };
                    // TODO: should it send response result to client?
                    // stream.write_all(b"response payload").unwrap();
                }
                Err(e) => error!("{:?}", e),
            }
        }
    }
}

fn extract_command(stream: &UnixStream) -> Command {
    let mut stream_buf = BufReader::new(stream);
    let mut recieve_buffer = String::new();

    loop {
        match stream_buf.read_line(&mut recieve_buffer) {
            Ok(s) => {
                if s == 0 {
                    break;
                }
            }
            Err(e) => {
                error!("read line failed... {:?}", e);
                break;
            }
        };
    }
    serde_json::from_str(&recieve_buffer).expect("parse failed")
}
