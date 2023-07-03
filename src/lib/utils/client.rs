use tokio::{io::AsyncWriteExt, net::TcpStream};

use super::log::{log, Log};

pub async fn spawn_client(address: &str) -> TcpStream {
    TcpStream::connect(address)
        .await
        .unwrap_or_else(|_| panic!("{}", log(Log::ClientConnectIssue(address.to_string()))))
}

pub async fn write_to_server(client: &mut TcpStream, content: impl Into<String>) {
    client.write_all(content.into().as_bytes()).await.unwrap();
}
