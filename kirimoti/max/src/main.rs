//!
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti
//! cargo new max
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti\max
//! cargo check
//! cargo run
//! 
//! [crates.io](https://crates.io/)
//! 

fn main() {
    // 4x4 table.
    let table = [
        [-1, 3, -2, 10]
        ,[16, -15, 9, -7]
        ,[8, -11, 4, 5]
        ,[-6, 9, 0, -1]
    ];

    let max = get_max(table);

    println!("{:?}, max={}", table, max);
}

fn get_max(table:[[i64;4];4]) -> i64 {
    let mut max = std::i64::MIN;
    for r in 0..table.len() {
        for c in 0..table[r].len() {
            if max < table[r][c] {
                max = table[r][c];
            }
        }
    }

    max
}