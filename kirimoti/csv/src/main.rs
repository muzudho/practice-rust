//!
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti
//! cargo new csv
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti\csv
//! cargo check
//! cargo run
//! 
//! [crates.io](https://crates.io/)
//! 

extern crate csv;

use std::*;

fn main() {
    let header : Vec<&str> = vec!["Apple", "Banana", "Cherry", "Daicon", "Egg"];
    
    // 列数は合わせる必要がある。
    let body : Vec<Vec<i64>> = vec![
        vec![10, 11, 12, 13, 14],
        vec![20, 21, 22, 23, 24],
        vec![30, 31, 32, 33, 34],
        vec![40, 41, 42, 43, 44],
        vec![50, 51, 52, 53, 54]];

    // let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut wtr = csv::Writer::from_path("table.csv").unwrap();

    wtr.write_record(
        header.iter().map(|v|v.to_string())
    );

    for row in body {
        wtr.write_record(
            row.iter().map(|v|v.to_string())
        );
    }

    wtr.flush();


    // let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut rdr = csv::Reader::from_path("table.csv").unwrap();

    for result in rdr.deserialize() {
        let record: Vec<i64> = result.unwrap();
        println!("Record: {:?}", record);
    }
}
