use std::os::unix::net::{UnixListener, UnixStream};
use std::fs;
use std::io::{Read, BufReader, BufRead};
use serde_json;

use client::Command;

pub const SOCKET_ADDR: &'static str = "/tmp/sync-memo.sock";

#[derive(Debug)]
pub struct Daemon {
    listener: UnixListener,
}

impl Daemon {
    pub fn new() -> Self {
        fs::remove_file(SOCKET_ADDR).unwrap_or(());
        Daemon {
          listener: UnixListener::bind(SOCKET_ADDR).expect("daemon start failed")
        }
    }
    
    pub fn listen(&self) {
        println!("[SERVER]: Waiting for connection from client...");
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let command = extract_command(&stream);
                    println!("Command: {:?}", command);
                    // let request_headers = extract_head(&stream);
                    // let response = create_response(&request_headers);
                    // stream.write_all(response.as_slice()).unwrap();
                }
                Err(e) => println!("{:?}", e)
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
            },
            Err(e) => {
                println!("read line failed... {:?}", e);
                break;
            },
        };
    }
    serde_json::from_str(&recieve_buffer).expect("parse failed")
}
