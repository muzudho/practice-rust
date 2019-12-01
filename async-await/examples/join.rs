// cd C:\Users\むずでょ\source\repos\practice-rust\async-await
// cargo check --example join
// cargo build --example join
// cargo run --example join
//
// See also:
// https://crates.io/
use futures::executor::block_on;
use std::thread;
use std::time::Duration;

async fn say_apple() {
    print!("app");
    thread::sleep(Duration::from_secs(1));
    println!("le!");
}

async fn say_banana() {
    print!("ban");
    thread::sleep(Duration::from_secs(1));
    println!("ana!");
}

async fn say_cherry() {
    print!("che");
    thread::sleep(Duration::from_secs(1));
    println!("rry!");
}

async fn say_mix_juice() {
    /*
    say_apple().await;
    say_banana().await;
    say_cherry().await;
    */
    futures::join!(say_apple(), say_banana(), say_cherry());
}

fn main() {
    let future = say_mix_juice();
    block_on(future);
}
