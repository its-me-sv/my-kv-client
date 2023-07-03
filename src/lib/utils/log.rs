use std::net::SocketAddr;

use chrono::Utc;

pub enum Log {
    ClientConnectIssue(String),
    ClientConnected(SocketAddr),
    ServerClosedConnection,
    ServerResponse(String),
    ReadError(String),
}

pub fn log(kind: Log) -> String {
    use Log::*;
    match kind {
        ClientConnectIssue(address) => {
            format!("[{}][CLIENT] Couldn't connect to {}", Utc::now(), address)
        }
        ClientConnected(address) => {
            format!(
                "[{}][CLIENT] Connected with IP {} at PORT {}",
                Utc::now(),
                address.ip(),
                address.port()
            )
        }
        ServerClosedConnection => {
            format!("[{}][CLIENT] Server closed connection", Utc::now())
        }
        ServerResponse(msg) => {
            format!("[{}][CLIENT] Recieved - {}", Utc::now(), msg)
        }
        ReadError(err) => {
            format!("[{}][CLIENT] Read error - {}", Utc::now(), err)
        }
    }
}
