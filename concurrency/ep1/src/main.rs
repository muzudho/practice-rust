/**
 * cargo new ep1
 * cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\concurrency\ep1
 * cargo run
 * 
 * [並行性](https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/concurrency.html)
 */
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        println!("Hello from a new thread!");
    });

    println!("Hello from a main thread!");
    thread::sleep(Duration::from_millis(20));
}
