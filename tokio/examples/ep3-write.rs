/*
 * cd C:\Users\むずでょ\source\repos\practice-rust\tokio
 * cargo check --example ep3-write
 * cargo build --example ep3-write
 * cargo run --example ep3-write
 *
 * [Struct tokio::net::TcpStream](https://docs.rs/tokio/0.2.3/tokio/net/struct.TcpStream.html)
 */
use futures::executor::block_on;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::prelude::*;

async fn connect() {
    // Sleep 3 seconds.
    println!("Info            | Please wait 1 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(1));

    // v4 か v6 かはサーバー側と合わせること。
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("Host            | {}", &addr);

    // クライアント側は リスナーではなく、ストリームを取得する。
    // https://docs.rs/tokio-tcp/0.1.2/src/tokio_tcp/stream.rs.html#49-58
    match TcpStream::connect(&addr).await {
        Ok(mut stream) => {
            match stream.set_nodelay(true) {
                Ok(_x) => {}
                Err(e) => panic!("{}", e),
            }

            println!("Connected from  | {:?}", stream.peer_addr().unwrap());

            // Write some data.
            match stream.write_all(b"hot dog!").await {
                Ok(_x) => {}
                Err(e) => panic!("{}", e),
            }
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
    println!("Info            | Please wait 1 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Info            | Finished.");
}
