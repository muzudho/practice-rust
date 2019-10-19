/**
 * cargo new ep1
 * cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\concurrency\ep1
 * cargo build --example channel-2
 * cargo run --example channel-2
 * 
 * [メッセージ受け渡しを使ってスレッド間でデータを転送する](https://doc.rust-jp.rs/book/second-edition/ch16-02-message-passing.html)
 */

use std::thread;
use std::sync::mpsc;

fn main() {
    let (childEnter, childExit) = mpsc::channel();
    let (mainEnter, mainExit) = mpsc::channel();

    let childHandle = thread::spawn(move || {
        let mut childBall = 1;
        childEnter.send(childBall).unwrap();

        childBall = mainExit.recv().unwrap();
        println!("Child           | Expected: 11, Got: {}.", childBall);
        childBall += 100;
        childEnter.send(childBall).unwrap();

        childBall = mainExit.recv().unwrap();
        println!("Child           | Expected: 1111, Got: {}, Finished.", childBall);
    });

    let mut mainBall = childExit.recv().unwrap();
    println!("Main            | Expected: 1, Got: {}.", mainBall);
    mainBall += 10;
    mainEnter.send(mainBall).unwrap();

    mainBall = childExit.recv().unwrap();
    println!("Main            | Expected: 111, Got: {}.", mainBall);
    mainBall += 1000;
    mainEnter.send(mainBall).unwrap();

    childHandle.join().unwrap();
    println!("Main            | Finished.");
}