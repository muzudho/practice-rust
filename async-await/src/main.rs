// cd C:\Users\むずでょ\source\repos\practice-rust
// cargo new async-await
//
// cd C:\Users\むずでょ\source\repos\practice-rust\async-await
// cargo check
// cargo build
// cargo run
//
// See also:
// https://crates.io/
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}
