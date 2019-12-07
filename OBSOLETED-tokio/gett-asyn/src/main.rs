//!
//! cargo new gett-asyn
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\tokio\gett-asyn
//! cargo check
//! cargo build
//! cargo run
//! 
//! [Getting asynchronous](https://tokio.rs/docs/futures/getting_asynchronous/)
//!

extern crate tokio;
extern crate futures;

use tokio::net::{TcpStream, tcp::ConnectFuture};
use futures::{Future, Async, Poll};

struct GetPeerAddr {
    connect: ConnectFuture,
}

impl Future for GetPeerAddr {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.connect.poll() {
            Ok(Async::Ready(socket)) => {
                println!("peer address = {}", socket.peer_addr().unwrap());
                Ok(Async::Ready(()))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => {
                println!("failed to connect\\(^q^)/: {}", e);
                Ok(Async::Ready(()))
            }
        }
    }
}

fn main() {
    let addr = "192.168.0.1:1234".parse().unwrap();
    // let addr = "127.0.0.1:9696".parse().unwrap();
    println!("Connect         | {}", addr);
    let connect_future = TcpStream::connect(&addr);
    let get_peer_addr = GetPeerAddr {
        connect: connect_future,
    };

    tokio::run(get_peer_addr);
}
