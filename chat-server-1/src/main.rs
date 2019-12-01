// cd C:\Users\むずでょ\source\repos\practice-rust
// cargo new chat-server-1
//
// cd C:\Users\むずでょ\source\repos\practice-rust\chat-server-1
// cargo check
// cargo build
// cargo run
//
// See also:
// https://crates.io/
// #[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;
use config::*;

fn main() {
    println!("Hello, world!");
    match read_toml("./config.toml".to_string()) {
        Ok(config) => {
            let host = config.host.unwrap();
            println!("Connect | {}:{}", host.domain.unwrap(), host.port.unwrap());
        }
        Err(err) => panic!(err),
    }
}
