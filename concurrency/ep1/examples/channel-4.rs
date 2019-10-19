/**
 * cargo new ep1
 * cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\concurrency\ep1
 * cargo check --example channel-4
 * cargo build --example channel-4
 * cargo run --example channel-4
 * 
 * [メッセージ受け渡しを使ってスレッド間でデータを転送する](https://doc.rust-jp.rs/book/second-edition/ch16-02-message-passing.html)
 */

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::io;

fn main() {
    // 2つの方向。
    let (child1Enter, child1Exit) = mpsc::channel();
    let (child2Enter, child2Exit) = mpsc::channel();
    let (mainEnter1, mainExit1) = mpsc::channel();
    let (mainEnter2, mainExit2) = mpsc::channel();

    // Sender と Receiver を move でクロージャーの中に入れる。
    let child1Handle = thread::spawn(move || {
        handle_child1(mainExit1, child1Enter)
    });

    let child2Handle = thread::spawn(move || {
        handle_child2(mainExit2, child2Enter)
    });

    println!("> Please input nature number. 0 is exit.");
    let mut line = String::new();

    // コマンド プロンプトからの入力があるまで待機します。
    io::stdin()
        .read_line(&mut line)
        .expect("info Failed to read_line"); // OKでなかった場合のエラーメッセージ。

    // 末尾の 改行 を除きます。前後の空白も消えます。
    line = line.trim().parse().expect("info Failed to parse");

    println!("Read            | {}", line);

    let mut mainBall:i64 = line.parse().unwrap();
    if mainBall==0 {

    } else {
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
    }

    // End.
    child1Handle.join().unwrap();
    child2Handle.join().unwrap();
    println!("Main            | Finished.");
}

fn handle_child1(mainExit1: Receiver<i64>, child1Enter: Sender<i64>)
{
    let mut childBall = mainExit1.recv().unwrap();
    if childBall==0 {

    } else {
        println!("Child1          | Expected: 1, Got: {}.", childBall);
        childBall += 10;
        child1Enter.send(childBall).unwrap();

        childBall = mainExit1.recv().unwrap();
        println!("Child           | Expected: 11111, Got: {}, Finished.", childBall);
        childBall += 100000;
        child1Enter.send(childBall).unwrap();
    }
}

fn handle_child2(mainExit2: Receiver<i64>, child2Enter: Sender<i64>)
{
    let mut childBall = mainExit2.recv().unwrap();
    if childBall==0 {

    } else {
        println!("Child           | Expected: 111, Got: {}.", childBall);
        childBall += 1000;
        child2Enter.send(childBall).unwrap();

        childBall = mainExit2.recv().unwrap();
        println!("Child           | Expected: 1111111, Got: {}, Finished.", childBall);
        childBall += 10000000;
        child2Enter.send(childBall).unwrap();
    }
}
