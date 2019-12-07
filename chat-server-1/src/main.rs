// cd C:\Users\むずでょ\source\repos\practice-rust
// cargo new chat-server-1
//
// cd C:\Users\むずでょ\source\repos\practice-rust\chat-server-1
// cargo check
// cargo build
// cargo run
//
// See also:
// https://crates.io/
// [CPerezz/rust-chat-server](https://github.com/CPerezz/rust-chat-server/blob/master/src/main.rs )

// #[macro_use]
extern crate serde_derive;
extern crate toml;

use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const MSG_SIZE: usize = 32;

mod config;
use config::*;

fn main() {
    println!("Hello, world!");
    match read_toml("./config.toml".to_string()) {
        Ok(config) => {
            let host = config.host.unwrap();
            let hostText = format!("{}:{}", host.domain.unwrap(), host.port.unwrap());
            println!("Host            | {}", hostText);

            //Instanciate the server
            let server = TcpListener::bind(&hostText).expect("Listener failet to bind.");

            //Set server in Non-blocking state to force it to hear for changes all the time.
            server
                .set_nonblocking(true)
                .expect("Failed to set Non-Blocking state.");

            //Generating a clients Storage Vector.
            let mut clients: Vec<std::net::TcpStream> = vec![];

            //An asynchronous, infinitely buffered channel. The channel function will return a (Sender, Receiver) tuple where all sends will be asynchronous.
            //Note that we've specified that Strings will be the types that travel through the channel.
            let (sender, reciever) = mpsc::channel::<String>();

            // クライアント待ち受け☆（＾～＾）
            loop {
                //If the recieved connection from the listener contains a correct value, is accepted.
                if let Ok((mut socket, addr)) = server.accept() {
                    println!("Client {:?} connected to the chhanel!", addr);

                    let sender = sender.clone();
                    //Clone the socket to push it into a thread.
                    clients.push(socket.try_clone().expect("Failed to clone the client."));

                    thread::spawn(move || loop {
                        //Create a buffer to store the msges.
                        let mut buff = vec![0; MSG_SIZE];

                        //Hear socket entries from sender an match it with a Result.
                        match socket.read(&mut buff) {
                            //a read() syscall on a socket that has been closed on the other end will return 0 bytes read,
                            //but no error, which should translate to Ok(0) in Rust.
                            //But this may only apply when the other end closed the connection cleanly.
                            Ok(0) => {
                                println!("\nClient: {} left the channel.", addr);
                                break;
                            }

                            //Handle when we do not read an empty socket
                            Ok(_) => {
                                //Set the buffer as an Iretartor and take it's elements while the condition retunrs true. Finally returns a Vec of type T
                                let msg = buff
                                    .clone()
                                    .into_iter()
                                    .take_while(|&x| x != 0)
                                    .collect::<Vec<_>>();

                                println!("\nMSG as Bytes:   {:?}", msg.clone());
                                let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                                println!("\n{}: {:?}", addr, msg);
                                sender.send(msg).expect("failed to send msg to reciever");
                            }
                            //Handle reading errors!
                            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                            Err(_) => {
                                println!("\nClient: {} left the channel.", addr);
                                break;
                            }
                        }

                        thread::sleep(Duration::from_millis(200));
                    });
                }
                //A bit of functionall programming to send the message to all the other chat members.
                if let Ok(msg) = reciever.try_recv() {
                    clients = clients
                        .into_iter()
                        .filter_map(|mut client| {
                            let buff = msg.clone().into_bytes();
                            buff.clone().resize(buff.len(), 0);

                            client.write_all(&buff).map(|_| client).ok()
                        })
                        .collect::<Vec<_>>();
                }

                thread::sleep(Duration::from_millis(200));
            }
        }
        Err(err) => panic!(err),
    }
}
