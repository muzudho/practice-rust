/**
 * cargo new ep1
 * cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\tokio\ep1
 * cargo check --example tokio-1
 * cargo build --example tokio-1
 * cargo run --example tokio-1
 * 
 * [Hello World!](https://tokio.rs/docs/getting-started/hello-world/)
 */
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

fn main() {
    // Parse the address of whatever server we're talking to
    let addr = "127.0.0.1:6142".parse().unwrap();

    let client = TcpStream::connect(&addr);

    println!("Info            | Finished.");
}
