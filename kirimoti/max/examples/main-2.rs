//!
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti\max
//! cargo check --example main-2
//! cargo run --example main-2
//! cargo check
//! cargo run
//! 
//! [crates.io](https://crates.io/)
//! 

fn main() {
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
    for row in table.iter() {
        for column in row.iter() {
            if max < *column {
                max = *column;
            }
        }
    }

    max
}
