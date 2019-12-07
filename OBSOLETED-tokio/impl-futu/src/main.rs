//!
//! cargo new ep1
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\tokio\impl-futu
//! cargo check
//! cargo build
//! cargo run
//! 
//! [Implementing futures](https://tokio.rs/docs/futures/basic/)
//!

extern crate tokio;
extern crate futures;

use futures::{Future, Async, Poll, try_ready};
use std::fmt;


struct HelloWorld;

impl Future for HelloWorld {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready("hello world".to_string()))
    }
}


struct Display<T>(T);

impl<T> Future for Display<T>
where
    T: Future,
    T::Item: fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), T::Error> {
        let value = try_ready!(self.0.poll());
        println!("{}", value);
        Ok(Async::Ready(()))
    }
}

fn main() {
    let future = Display(HelloWorld);
    tokio::run(future);
}

