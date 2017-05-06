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

type LogLevel = &'static str;
pub const LOG_INFO: LogLevel = "info";
pub const LOG_ERROR: LogLevel = "error";

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Info(String),
    Error(String),
}

impl Response {
    pub fn from_log_level(log_level: LogLevel, payload: String) -> Self {
        match log_level {
            LOG_INFO => Response::Info(payload),
            _ => Response::Error(payload),
        }
    }

    // TODO: Perhaps it can return &[u8]
    pub fn to_chunk<'a>(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("parse failed")
    }

    pub fn write_log(&self) {
        match self {
            &Response::Info(ref payload) => {
                info!("{}", payload);
                println!("{}", payload);
            },
            &Response::Error(ref payload) => {
                error!("{}", payload);
                println!("{}", payload);
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

    // TODO: Enable to recieve notification from daemon process
}
