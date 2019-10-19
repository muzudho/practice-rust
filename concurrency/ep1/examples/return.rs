/**
 * cargo new ep1
 * cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\concurrency\ep1
 * cargo build --example return
 * cargo run --example return
 * 
 * [並行性](https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/concurrency.html)
 */

use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        "Hello from a new thread!"
    });

    println!("Main            | Child said: {}", handle.join().unwrap());
}
