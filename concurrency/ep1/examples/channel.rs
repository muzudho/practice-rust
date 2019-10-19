/**
 * cargo new ep1
 * cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\concurrency\ep1
 * cargo build --example channel
 * cargo run --example channel
 * 
 * [メッセージ受け渡しを使ってスレッド間でデータを転送する](https://doc.rust-jp.rs/book/second-edition/ch16-02-message-passing.html)
 */

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    // 値は{}です
    println!("Main            | Got: {}", received);
}