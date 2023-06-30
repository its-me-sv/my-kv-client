use std::io::{stdin, Read, Write};
use std::net::TcpStream;

const ADDRESS: &str = "127.0.0.1:8080";

fn main() {
    let mut stream = TcpStream::connect(ADDRESS)
        .unwrap_or_else(|_| panic!("[CLIENT] Couldn't bind to {ADDRESS}"));
    println!("[CLIENT] Connected to server binded at {ADDRESS}");

    loop {
        let mut input = String::new();
        println!("Input:");
        stdin().read_line(&mut input).expect("Failed to read input");

        stream.write_all(input.trim().as_bytes()).unwrap();

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    println!("[Client] Server closed connection");
                    break;
                }
                let response = String::from_utf8_lossy(&buffer[..size]).to_string();
                println!("[CLIENT] Received - {response}");
            }
            Err(e) => {
                eprintln!("[CLIENT] Error: {e} | Read error occurred");
                break;
            }
        }
    }
}
