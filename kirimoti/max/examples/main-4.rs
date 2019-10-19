//!
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti\max
//! cargo check --example main-4
//! cargo run --example main-4
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

    let get_max = |cell, relay|{
        if cell < relay {
            relay
        } else {
            cell
        }
    };

    let max = each_cell(
        &table,
        std::i64::MIN,
        get_max);

    println!("{:?}, max={}", table, max);
}

fn each_cell<F>(table:&Vec<Vec<i64>>, init_value:i64, callback:F) -> i64
    where F : Fn(i64, i64) -> i64 {

    let mut relay = init_value;
    for row in table.iter() {
        for column in row.iter() {
            relay = callback(*column, relay);
        }
    }

    relay
}
