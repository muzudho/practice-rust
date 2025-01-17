// Person.toml は Cargo.toml が置いてあるディレクトリに置けだぜ☆（＾～＾）
//
// cd C:\Users\むずでょ\source\repos\practice-rust
// cargo new toml
//
// C:\Users\むずでょ\source\repos\practice-rust\toml
// cargo check
// cargo build
// cargo run
//
// TOMLを使ってみようぜ☆（＾～＾）？
// https://cipepser.hatenablog.com/entry/rust-toml
// https://crates.io/crates

#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs;
use std::io::{BufReader, Read};

fn main() {

    println!("Hello, world!");

    let s = match read_file("./Person.toml".to_owned()) {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };

    let person: Result<Person, toml::de::Error> = toml::from_str(&s);
    match person {
        Ok(p) => println!("{:#?}", p),
        Err(e) => panic!("fail to parse toml: {}", e),
    };
}

#[derive(Debug, Deserialize)]
struct Person {
    profile: Option<Profile>,
}

#[derive(Debug, Deserialize)]
struct Profile {
    name: Option<String>,
    age: Option<i32>,
}

fn read_file(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}
