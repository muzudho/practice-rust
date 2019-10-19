/**
 * cargo new ep1
 * cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\concurrency\ep1
 * cargo check --example input
 * cargo build --example input
 * cargo run --example input
 */
use std::io;

fn main() {
    let mut line = String::new();

    // コマンド プロンプトからの入力があるまで待機します。
    io::stdin()
        .read_line(&mut line)
        .expect("info Failed to read_line"); // OKでなかった場合のエラーメッセージ。

    // 末尾の 改行 を除きます。前後の空白も消えます。
    line = line.trim().parse().expect("info Failed to parse");

    println!("Read            | {}", line);
}