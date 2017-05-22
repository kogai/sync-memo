use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use serde_json;

use daemon::SOCKET_ADDR;

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Add(Vec<String>),
    Show,
    Kill,
}

type LogLevel = &'static str;
pub const LOG_INFO: LogLevel = "info";
pub const LOG_ERROR: LogLevel = "error";

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Info(String),
    Error(String),
}

impl Response {
    pub fn with_info(payload: &str) -> Self {
        Response::with_log_level(LOG_INFO, payload)
    }

    pub fn with_error(payload: &str) -> Self {
        Response::with_log_level(LOG_ERROR, payload)
    }

    pub fn with_log_level(log_level: LogLevel, payload: &str) -> Self {
        match log_level {
            LOG_INFO => Response::Info(payload.to_owned()),
            _ => Response::Error(payload.to_owned()),
        }
    }

    // TODO: Perhaps it can return &[u8]
    pub fn to_string<'a>(&self) -> String {
        serde_json::to_string(self).expect("parse failed")
    }

    pub fn write_log(&self) {
        match self {
            &Response::Info(ref payload) => {
                info!("{}", payload);
            }
            &Response::Error(ref payload) => {
                error!("{}", payload);
            }
        }
    }
}

#[derive(Debug)]
pub struct Client {
    socket: &'static str,
}

impl Client {
    pub fn new() -> Self {
        Client { socket: SOCKET_ADDR }
    }

    pub fn send(&self, command: Command) -> Response {
        let mut connection = match UnixStream::connect(self.socket) {
            Ok(socket) => socket,
            Err(e) => {
                error!("{}", e);
                return Response::with_error(&format!("{}", e));
            }
        };
        let payload = serde_json::to_string(&command).expect("parse failed");
        connection.write_all(payload.as_bytes()).unwrap();

        let mut buffer = Vec::new();
        connection.read_to_end(&mut buffer).unwrap();
        serde_json::from_slice(buffer.as_slice()).expect("parse response failed")
    }
}

mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn it_should_send_request_through_socket() {
        let (mut client, mut server) = UnixStream::pair().unwrap();

        let handler = thread::spawn(move || {
            let mut buffer = [1; 20];
            server
                .read(&mut buffer)
                .expect("server couldnt read request");
            let buffer = String::from_utf8(buffer.to_vec()).unwrap();
            assert_eq!(buffer, "message from client\u{1}");

            server
                .write_all(b"message from server")
                .expect("server couldn't write request");
        });

        client.write_all(b"message from client").unwrap();

        let mut buffer = String::new();
        client
            .read_to_string(&mut buffer)
            .expect("client couldnt read request");
        assert_eq!(buffer, "message from server");

        handler.join().unwrap();
    }
}

