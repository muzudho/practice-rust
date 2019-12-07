//!
//! Echo server - poppo.
//!
//! A simple server that writes "poppo\n" when reads any.
//!
//! cd C:\Users\むずでょ\source\repos\practice-rust\echo-server
//! cargo check
//! cargo build
//! cargo run
//!
//! See: [Rustにっき/8日目・TCPサーバ](https://cha-shu00.hatenablog.com/entry/2019/03/02/174532)
//!
extern crate serde_derive;
extern crate toml;

use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::thread;

mod config;
use config::*;

fn main() {
    match read_toml("./config.toml".to_string()) {
        Ok(config) => {
            let host = config.host.unwrap();
            let host_text = format!("{}:{}", host.domain.unwrap(), host.port.unwrap());
            println!("Host            | {}", host_text);
            let mut addrs = host_text.to_socket_addrs().unwrap();

            // Change to ip v4.
            if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
                // Server standup  | 127.0.0.1:9696
                println!("Server standup  | {}", addr);

                // Wait for connection.
                let listener = TcpListener::bind(addr).expect("Error. failed to bind.");

                for streams in listener.incoming() {
                    match streams {
                        Err(e) => eprintln!("error: {}", e),
                        Ok(stream) => {
                            // Create the thread.
                            thread::spawn(move || {
                                handle_client(stream)
                                    .unwrap_or_else(|error| eprintln!("{:?}", error));
                            });
                        }
                    }
                }
            } else {
                eprintln!("Invalid Host:Port Number");
            }
        }
        Err(err) => panic!(err),
    }
}

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    println!("Connection from | {}", stream.peer_addr()?);

    // Buffering.
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let mut line = String::new();

    loop {
        println!("Info            | Waiting...");
        if let Err(err) = reader.read_line(&mut line) {
            // panic!("error during receive a line: {}", err);
            println!("Error           | {}", err);
        }
        println!("Read            | {}", line);

        if line == "quit" {
            return Ok(());
        }

        line.clear();

        // 改行で送信完了。
        let msg = "poppo";
        println!("Write           | {}", msg);
        writer.write(format!("{}\n", msg).as_bytes())?;
        writer.flush()?;
    }
}
