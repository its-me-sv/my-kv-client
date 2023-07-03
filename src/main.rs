use client::utils::{
    client::{spawn_client, write_to_server},
    input::get_input,
    log::{log, Log},
};
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:8080";
    let mut client = spawn_client(address).await;

    println!(
        "{}",
        log(Log::ClientConnected(client.local_addr().unwrap()))
    );

    loop {
        let user_input = get_input();
        write_to_server(&mut client, user_input).await;

        let mut buffer = [0; 1024];
        match client.read(&mut buffer).await {
            Ok(size) => {
                if size == 0 {
                    println!("{}", log(Log::ServerClosedConnection));
                    break;
                }
                let response = String::from_utf8_lossy(&buffer[..size]).to_string();
                log(Log::ServerResponse(response));
            }
            Err(e) => {
                eprintln!("{}", log(Log::ReadError(e.to_string())));
                break;
            }
        }
    }
}
