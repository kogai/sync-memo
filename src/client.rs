use std::io::Write;
use std::os::unix::net::UnixStream;
use serde_json;

use daemon::SOCKET_ADDR;

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Add(Vec<String>),
    Show,
    Kill,
}

#[derive(Debug)]
pub struct Client {
    socket: &'static str,
}

impl Client {
    pub fn new() -> Self {
        Client { socket: SOCKET_ADDR }
    }

    pub fn send(&self, command: Command) {
        let mut connection = match UnixStream::connect(self.socket) {
            Ok(socket) => socket,
            Err(e) => {
                error!("{}", e);
                return ();
            }
        };
        let payload = serde_json::to_string(&command).expect("parse failed");
        connection.write_all(payload.as_bytes()).unwrap();

        // TODO: perhaps it should handle response from daemon
        // let mut buffer = Vec::new();
        // connection.read_to_end(&mut buffer).unwrap();
        // String::from_utf8(buffer).unwrap()
    }
}
