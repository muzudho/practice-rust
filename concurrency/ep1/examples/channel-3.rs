/**
 * cargo new ep1
 * cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\concurrency\ep1
 * cargo build --example channel-3
 * cargo run --example channel-3
 * 
 * [メッセージ受け渡しを使ってスレッド間でデータを転送する](https://doc.rust-jp.rs/book/second-edition/ch16-02-message-passing.html)
 */

use std::thread;
use std::sync::mpsc;

fn main() {
    // 2つの方向。
    let (child1Enter, child1Exit) = mpsc::channel();
    let (child2Enter, child2Exit) = mpsc::channel();
    let (mainEnter1, mainExit1) = mpsc::channel();
    let (mainEnter2, mainExit2) = mpsc::channel();

    let child1Handle = thread::spawn(move || {
        let mut childBall = mainExit1.recv().unwrap();
        println!("Child1          | Expected: 1, Got: {}.", childBall);
        childBall += 10;
        child1Enter.send(childBall).unwrap();

        childBall = mainExit1.recv().unwrap();
        println!("Child           | Expected: 11111, Got: {}, Finished.", childBall);
        childBall += 100000;
        child1Enter.send(childBall).unwrap();
    });

    let child2Handle = thread::spawn(move || {
        let mut childBall = mainExit2.recv().unwrap();
        println!("Child           | Expected: 111, Got: {}.", childBall);
        childBall += 1000;
        child2Enter.send(childBall).unwrap();

        childBall = mainExit2.recv().unwrap();
        println!("Child           | Expected: 1111111, Got: {}, Finished.", childBall);
        childBall += 10000000;
        child2Enter.send(childBall).unwrap();
    });

    let mut mainBall = 1;
    mainEnter1.send(mainBall).unwrap();

    mainBall = child1Exit.recv().unwrap();
    println!("Main            | Expected: 11, Got: {}.", mainBall);
    mainBall += 100;
    mainEnter2.send(mainBall).unwrap();

    mainBall = child2Exit.recv().unwrap();
    println!("Main            | Expected: 1111, Got: {}.", mainBall);
    mainBall += 10000;
    mainEnter1.send(mainBall).unwrap();

    mainBall = child1Exit.recv().unwrap();
    println!("Main            | Expected: 111111, Got: {}.", mainBall);
    mainBall += 1000000;
    mainEnter2.send(mainBall).unwrap();

    mainBall = child2Exit.recv().unwrap();
    println!("Main            | Expected: 11111111, Got: {}.", mainBall);

    child1Handle.join().unwrap();
    child2Handle.join().unwrap();
    println!("Main            | Finished.");
}