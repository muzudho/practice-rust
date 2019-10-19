/**
 * cargo new ep1
 * cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\concurrency\ep1
 * cargo build --example data-race-1
 * cargo run --example data-race-1
 * 
 * [並行性](https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/concurrency.html)
 */

use std::thread;
use std::time::Duration;

fn main() {
    let mut data = 1;

    let handle = thread::spawn(move || {
        data += 1;
        println!("Child           | data: {}", data);
    });

    handle.join().unwrap();
    println!("Main            | data: {}", data);
    thread::sleep(Duration::from_millis(50));
}