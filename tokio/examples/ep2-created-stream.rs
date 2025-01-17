//! Hello world server.
//!
//! A simple client that opens a TCP stream, writes "hello world\n", and closes
//! the connection.
//!
//! You can test this out by running:
//!
//!     ncat -l 6142
//!     nc.exe -l -p 6142 -t -e cmd.exe
//!
//! And then in another terminal run:
//!
//!     cargo run --example hello_world

/*
 * cd C:\Users\むずでょ\source\repos\practice-rust\tokio
 * cargo check --example ep2-created-stream
 * cargo build --example ep2-created-stream
 * cargo run --example ep2-created-stream
 *
 * [Hello World!](https://tokio.rs/docs/getting-started/hello-world/)
 * [tokio 0.2.3](https://docs.rs/crate/tokio/0.2.3)
 */
use futures::executor::block_on;
use std::net::SocketAddr;

async fn connect() {
    // Sleep 3 seconds.
    println!("Info            | Please wait 1 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(1));

    // let addr: SocketAddr = "localhost:3000".parse().unwrap(); // v4 or v6?
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("Host            | {}", &addr);

    // https://docs.rs/tokio-tcp/0.1.2/src/tokio_tcp/stream.rs.html#49-58
    match tokio::net::TcpStream::connect(&addr).await {
        Ok(_stream) => {
            // Following snippets come here...
            println!("Info            | Connect end.");
        }
        Err(e) => println!("{:?}", e),
    };
}

#[tokio::main]
async fn main() {
    println!("Info            | Please wait 1 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(1));

    // syncronized.
    block_on(connect());
    // asyncronized.
    // tokio::spawn(connect());

    // Sleep 9 seconds.
    println!("Info            | Please wait 3 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Info            | Finished.");
}
