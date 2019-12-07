// cd C:\Users\むずでょ\source\repos\practice-rust
// cargo new chat-client-1
//
// cd C:\Users\むずでょ\source\repos\practice-rust\chat-client-1
// cargo check
// cargo build
// cargo run
//
// See also:
// https://crates.io/

// #[macro_use]
extern crate serde_derive;
extern crate toml;

// TcpListener は、標準のと Tokio のとがあって、どちらか選べだぜ☆（＾～＾）
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::net::TcpStream;

mod config;
use config::*;

fn main() {
    println!("I am a client!");
    match read_toml("./config.toml".to_string()) {
        Ok(config) => {
            let host = config.host.unwrap();
            let host_text = format!("{}:{}", host.domain.unwrap(), host.port.unwrap());
            println!("Host            | {}", host_text);

            let client = TcpStream::connect(&host_text)
                .and_then(move |stream| {
                    stream.set_nodelay(true);
                    println!("Connected from | {}", stream.peer_addr()?);

                    // Buffering.
                    let mut reader = BufReader::new(&stream);
                    let mut writer = BufWriter::new(&stream);

                    loop {
                        // Standard input
                        let mut line = String::new();
                        std::io::stdin().read_line(&mut line).ok();
                        // 改行を削る。
                        line = line.replace("\r", "\n").replace("\n", "");

                        println!("Input           | [{}]", line);
                        match line.as_str() {
                            "exit" => {
                                break;
                            }
                            "" => {
                                // 空打ちで、サーバーのメッセージ読取。
                                if let Err(err) = reader.read_line(&mut line) {
                                    // panic!("error during receive a line: {}", err);
                                    println!("Error           | {}", err);
                                }
                                println!("Read            | {}", line);
                            }
                            _ => {
                                // その他は、サーバーへのメッセージ送信。
                                println!("Write           | {}", line);
                                writer.write(format!("{}\n", line).as_bytes())?;
                                writer.flush()?;
                            }
                        }
                    }

                    Ok(())
                })
                .map_err(|err| {
                    println!("connection error = {:?}", err);
                });
        }
        Err(err) => panic!(err),
    }
}
