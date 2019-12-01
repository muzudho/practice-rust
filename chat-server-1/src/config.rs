use serde_derive::*;
use std::fs;
use std::io::{BufReader, Read};
// use toml::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: Option<Host>,
}

#[derive(Debug, Deserialize)]
pub struct Host {
    pub domain: Option<String>,
    pub port: Option<u16>,
}

pub fn read_toml(path: String) -> Result<Config, toml::de::Error> {
    let contents = match read_file(path.to_owned()) {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };

    println!("contents:\n----\n{}\n----", contents);

    toml::from_str(&contents)
}

fn read_file(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut file_read = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    file_read
        .read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}
