use std::os::unix::net::{UnixListener, UnixStream};
use std::fs;
use std::io::Read;

#[derive(Debug)]
pub struct Daemon {
    listener: UnixListener,
}

const SOCKET_ADDR: &'static str = "/tmp/sync-memo.sock";

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
            println!("{:?}", stream);
            /*
            match stream {
                Ok(mut stream) => {
                    spawn(move || {
                        println!("[SERVER]: Recieve connection from client. {:?}", stream);
                        let request_headers = extract_head(&stream);
                        let response = create_response(&request_headers);
                        stream.write_all(response.as_slice()).unwrap();
                    });
                }
                Err(e) => println!("{:?}", e)
            }
            */
        }
    } 
}
