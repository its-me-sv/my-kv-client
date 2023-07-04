use std::io::{Read, Write};
use std::net::TcpStream;

use super::log::{log, Log};

pub fn spawn_client(address: &str) -> TcpStream {
    TcpStream::connect(address)
        .unwrap_or_else(|_| panic!("{}", log(Log::ClientConnectIssue(address.to_string()))))
}

pub fn write_to_server(client: &mut TcpStream, content: impl Into<String>) {
    client.write_all(content.into().as_bytes()).unwrap();
}

pub fn read_from_server(client: &mut TcpStream) -> Log {
    let mut buffer = [0; 1024];
    match client.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                return Log::ServerClosedConnection;
            }
            let response = String::from_utf8_lossy(&buffer[..size]).to_string();
            Log::ServerResponse(response)
        }
        Err(e) => Log::ReadError(e.to_string()),
    }
}
