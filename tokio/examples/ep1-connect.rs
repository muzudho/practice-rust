use std::net::SocketAddr;
/**
 * cd C:\Users\むずでょ\source\repos\practice-rust\tokio
 * cargo check --example ep1-connect
 * cargo build --example ep1-connect
 * cargo run --example ep1-connect
 *
 * [Hello World!](https://tokio.rs/docs/getting-started/hello-world/)
 */
use tokio::net::TcpStream;

fn main() {
    // Parse the address of whatever server we're talking to
    let addr: SocketAddr = "127.0.0.1:6142".parse().unwrap();
    println!("Host            | {}", &addr);

    let _client = TcpStream::connect(&addr);

    println!("Info            | Finished.");
}
