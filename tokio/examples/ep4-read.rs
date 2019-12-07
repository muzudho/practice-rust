/*
 * cd C:\Users\むずでょ\source\repos\practice-rust\tokio
 * cargo check --example ep4-read
 * cargo build --example ep4-read
 * cargo run --example ep4-read
 *
 * [Reading and Writing Data](https://tokio.rs/docs/io/reading_writing_data/)
 */
use futures::executor::block_on;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::str;
use tokio::net::TcpStream;
use tokio::prelude::*;

async fn connect() {
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

            loop {
                // 改行を付けないと、受信側が 受信完了しません。
                match stream.write_all("hot dog!\n".as_bytes()).await {
                    Ok(_x) => {
                        stream.flush();
                        println!("Info            | Writed.");
                    }
                    Err(e) => panic!("{}", e),
                }

                println!("Info            | Waiting for read.");
                // Read.
                // https://docs.rs/tokio/0.1.12/tokio/prelude/trait.Read.html#tymethod.read
                let mut buffer = [0; 2048];

                // 末尾の改行をもって受信完了。
                match stream.read(&mut buffer[..]).await {
                    Ok(size) => {
                        println!(
                            "Read            | {} | {:?}",
                            size,
                            str::from_utf8(&buffer[..size]).unwrap()
                        );
                    }
                    Err(e) => panic!(e),
                }

                // Sleep 3 seconds.
                println!("Info            | Please wait 3 seconds.");
                std::thread::sleep(std::time::Duration::from_secs(3));
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
    println!("Info            | Please wait 10 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("Info            | Finished.");
}
