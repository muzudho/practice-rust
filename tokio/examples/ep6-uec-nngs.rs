/*
 * cd C:\Users\むずでょ\source\repos\practice-rust\tokio
 * set RUST_BACKTRACE=full
 * cargo check --example ep6-uec-nngs
 * cargo build --example ep6-uec-nngs
 * cargo run --example ep6-uec-nngs
 *
 * [Reading and Writing Data](https://tokio.rs/docs/io/reading_writing_data/)
 */
mod config;

use config::*;
use encoding_rs::*;
use futures::executor::block_on;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::prelude::*;

async fn connect() {
    match read_toml("./config-uec.toml".to_string()) {
        Ok(conf) => {
            // v4 か v6 かはサーバー側と合わせること。名前解決はしないので、数で打ち込むこと。
            let host = conf.host.unwrap();
            // Example: "127.0.0.1:3000"
            let host_text = format!("{}:{}", host.domain.unwrap(), host.port.unwrap());
            println!("Trace   | hostText=[{}]", host_text);
            let addr: SocketAddr = match host_text.parse() {
                Ok(x) => x,
                Err(e) => panic!("Error   | socketFail=>{}", e),
            };
            println!("Trace   | host=[{}]", &addr);

            // サーバー・プログラムなら TCPリスナーを作成し、クライアント・プログラムなら TCPストリームを取得する。
            // https://docs.rs/tokio-tcp/0.1.2/src/tokio_tcp/stream.rs.html#49-58
            match TcpStream::connect(&addr).await {
                Ok(mut stream) => {
                    match stream.set_nodelay(true) {
                        Ok(_x) => {}
                        Err(e) => panic!("Error   | nodelayFail=[{}]", e),
                    }

                    println!("Trace   | connectedTo=[{:?}]", stream.peer_addr().unwrap());
                    println!("--------+-------------------");
                    println!("Trace   | We go to the NNGS!");

                    loop {
                        // 標準入力。
                        println!("Trace   | Please key typing.");
                        let mut line = String::new();
                        std::io::stdin().read_line(&mut line).ok();
                        // 改行を削る。
                        line = line.replace("\r", "\n").replace("\n", "");
                        println!("Trace   | input=[{}]", line);

                        match line.as_str() {
                            "exit" => {
                                // ループから抜けます。このコマンドはサーバー側には送れません。
                                break;
                            }
                            "r" => {
                                // `r` コマンドで、サーバーのメッセージ読取。
                                // この状態からは途中で抜けれません。
                                println!("Trace   | Waiting for read.");
                                // Read.
                                // https://docs.rs/tokio/0.1.12/tokio/prelude/trait.Read.html#tymethod.read
                                let mut buffer = [0; 2048];
                                // 末尾の改行をもって受信完了。
                                match stream.read(&mut buffer[..]).await {
                                    Ok(size) => {
                                        println!("Trace   | readSize=[{}]", size);
                                        println!("Trace   | read=[{:?}]", &buffer[..size]);

                                        // サーバーから送られてくるのは Shift-JIS かも知れない。変換する。
                                        let (cow, encoding_used, had_errors) =
                                            SHIFT_JIS.decode(&buffer[..size]);
                                        let utf8_text = format!("{}", &cow[..]);
                                        println!("Trace   | read=[{}]", utf8_text);
                                        stream.flush();
                                    }
                                    Err(e) => panic!("Error   | readFail=>{}", e),
                                }
                            }
                            _ => {
                                // その他は、サーバーへのメッセージ送信。
                                println!("Trace   | write=>{}", line);
                                // 改行を付けてください。受信側が 受信完了するために必要です。
                                match stream.write_all(format!("{}\n", line).as_bytes()).await {
                                    Ok(_x) => {
                                        stream.flush();
                                        println!("Trace   | Writed.");
                                    }
                                    Err(e) => panic!("Error   | writeFail=>{}", e),
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("Error   | connectFail=[{:?}]", e),
            };
        }
        Err(e) => panic!("Error   | configFail=[{}]", e),
    }
}

#[tokio::main]
async fn main() {
    println!("Trace   | Please wait 1 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(1));

    // syncronized.
    block_on(connect());
    // asyncronized.
    // tokio::spawn(connect());

    // Sleep 9 seconds.
    println!("Trace   | Please wait 10 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("Trace   | Finished.");
}
