//!
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti\max
//! cargo check --example main-3
//! cargo run --example main-3
//! 
//! [crates.io](https://crates.io/)
//! 

fn main() {
    let table = vec![
        vec![-1, 3, -2, 10]
        ,vec![16, -15, 9, -7]
        ,vec![8, -11, 4, 5]
        ,vec![-6, 9, 0, -1]
    ];

    let max = get_max(&table);

    println!("{:?}, max={}", table, max);
}

fn get_max(table:&Vec<Vec<i64>>) -> i64 {
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
