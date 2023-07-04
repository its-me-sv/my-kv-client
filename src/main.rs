use client::utils::{
    client::{read_from_server, spawn_client, write_to_server},
    input::get_input,
    log::{log, Log},
};

fn main() {
    let address = "127.0.0.1:8080";
    let mut client = spawn_client(address);

    println!(
        "{}",
        log(Log::ClientConnected(client.local_addr().unwrap()))
    );

    loop {
        let user_input = get_input();
        write_to_server(&mut client, user_input);

        match read_from_server(&mut client) {
            Log::ServerClosedConnection => {
                println!("{}", log(Log::ServerClosedConnection));
                break;
            }
            resp => println!("{}", log(resp)),
        }
    }
}
