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
        println!("[SERVER]: Waiting for connection from client...");
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    use client::Command::*;
                    let command = extract_command(&stream);
                    match command {
                        Add(file_names) => {
                            // for file_name in file_names {
                            //     file_handler.add_files(file_name);
                            // }
                        }
                        Show => {
                            let file_ids = self.file_handler.get_file_ids();
                            // TODO: speed up with concurrent request
                            let gists = file_ids.into_iter()
                                .map(|id| get_gist(&id))
                                .collect::<Vec<_>>();
                            // TODO: pretifier result
                            println!("results: {:?}", gists);
                        }
                        Kill => exit(1),
                    };
                    // TODO: should it send response result to client?
                    // let response = create_response(&request_headers);
                    // stream.write_all(response.as_slice()).unwrap();
                }
                Err(e) => println!("{:?}", e),
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
                println!("read line failed... {:?}", e);
                break;
            }
        };
    }
    serde_json::from_str(&recieve_buffer).expect("parse failed")
}
