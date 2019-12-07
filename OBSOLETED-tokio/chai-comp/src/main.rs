//!
//! cargo new chai-comp
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\tokio\chai-comp
//! cargo check
//! cargo build
//! cargo run
//! 
//! [Chaining computations](https://tokio.rs/docs/futures/getting_asynchronous/)
//!

extern crate tokio;
extern crate bytes;
#[macro_use]
extern crate futures;

use tokio::io::AsyncWrite;
use tokio::net::{TcpStream, tcp::ConnectFuture};
use bytes::{Bytes, Buf};
use futures::{Future, Async, Poll};
use std::io::{self, Cursor};
use std::thread;
use std::time::Duration;

// HelloWorld has two states, namely waiting to connect to the socket
// and already connected to the socket
enum HelloWorld {
    Connecting(ConnectFuture),
    Connected(TcpStream, Cursor<Bytes>),
}

impl Future for HelloWorld {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        use self::HelloWorld::*;

        loop {
            match self {
                Connecting(ref mut f) => {
                    let socket = try_ready!(f.poll());
                    let data = Cursor::new(Bytes::from_static(b"h\ne\nl\nl\no\n \nw\no\nr\nl\nd\nquit\n"));
                    *self = Connected(socket, data);
                }
                Connected(ref mut socket, ref mut data) => {
                    // Keep trying to write the buffer to the socket as long as the
                    // buffer has more bytes available for consumption
                    while data.has_remaining() {
                        try_ready!(socket.write_buf(data));
                    }

                    try_ready!(socket.shutdown());

                    return Ok(Async::Ready(()));
                }
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1:9696".parse().unwrap();
    // let addr = "127.0.0.1:1234".parse().unwrap();

    let connect_future = TcpStream::connect(&addr);
    let hello_world = HelloWorld::Connecting(connect_future);

    // Run it, here we map the error since tokio::run expects a Future<Item=(), Error=()>
    tokio::run(hello_world.map_err(|e| println!("{0}", e)));

    thread::sleep(Duration::from_millis(10000));
    println!("Info            | Finished.");
}