/*
 * cd C:\Users\むずでょ\source\repos\practice-rust\tokio
 * cargo check --example ep5-input-read
 * cargo build --example ep5-input-read
 * cargo run --example ep5-input-read
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
                // 標準入力。
                println!("Info            | Please key typing.");
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).ok();
                // 改行を削る。
                line = line.replace("\r", "\n").replace("\n", "");
                println!("Input           | [{}]", line);

                match line.as_str() {
                    "exit" => {
                        // ループから抜けます。このコマンドはサーバー側には送れません。
                        break;
                    }
                    "" => {
                        // 空打ちで、サーバーのメッセージ読取。
                        // この状態からは途中で抜けれません。
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
                    }
                    _ => {
                        // その他は、サーバーへのメッセージ送信。
                        println!("Write           | {}", line);
                        // 改行を付けてください。受信側が 受信完了するために必要です。
                        match stream.write_all(format!("{}\n", line).as_bytes()).await {
                            Ok(_x) => {
                                stream.flush();
                                println!("Info            | Writed.");
                            }
                            Err(e) => panic!("{}", e),
                        }
                    }
                }
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
