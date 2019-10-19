//!
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti
//! cargo new pineapple
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti\pineapple
//! cargo check
//! cargo run
//! 
//! [crates.io](https://crates.io/)
//! 

extern crate csv;

use std::fs;
use rand::prelude::*;

fn main() {
    /* // Making of -.
    let mut hiragana = "あいうえお　かきくけこ　さしすせそ　たちつてと　なにぬねの　はひふへほ　まみむめも　やゆよ　らりるれろ　わをん
        がぎぐげご　ざじずぜぞ　だぢづでど　ばびぶべぼ　ぱぴぷぺぽ　ゃゅょ　ヴ
        ぁぃぅぇぉ　っー".to_string();
    hiragana.retain(|c| c != ' ' && c != '　' && c != '\r' && c != '\n');
    println!("let hiragana = \"{}\".to_string();",hiragana);
    */
    let hiragana = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよわおんがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽゃゅょヴぁぃぅぇぉっー".to_string();

    /* // Making of -.
    let mut katakana = "アイウエオ　カキクケコ　サシスセソ　タチツテト　ナニヌネノ　ハヒフヘホ　マミムメモ　ヤユヨ　ラリルレロ　ワヲン
        ガギグゲゴ　ザジズゼゾ　ダヂヅデド　バビブベボ　パピプペポ　ャュョ　ヴ
        ァィゥェォ　っ～".to_string();
    katakana.retain(|c| c != ' ' && c != '　' && c != '\r' && c != '\n');
    println!("let katakana = \"{}\".to_string();",katakana);
    */
    let katakana = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨワオンガギグゲゴザジズゼゾダヂヅデドバビブベボパピプペポャュョヴァィゥェォッ～".to_string();

    let multibyte_hiragana: Vec<char> = hiragana.chars().collect();
    let multibyte_katakana: Vec<char> = katakana.chars().collect();

    let mut table: Vec<Vec<i64>> = read_table(&multibyte_hiragana);


    let two_kanas_vec = match read_text() {
        Some(contents) => {
            // テキスト・ファイル入力。
            get_2_kana_vec(&multibyte_hiragana, &multibyte_katakana, &contents)
        }
        None => {
            // 手入力。
            get_input_2_kana_vec(&multibyte_hiragana, &multibyte_katakana)
        }
    };

    let bonus = multibyte_hiragana.len() as i64;
    for (ch1, ch2) in two_kanas_vec {

        let index1 = index_of(&ch1, &multibyte_hiragana, &multibyte_katakana);
        let index2 = index_of(&ch2, &multibyte_hiragana, &multibyte_katakana);
        if let Some(y) = index1 {
            if let Some(x) = index2 {
                println!("{}{} {}{}", ch1, y, ch2, x);

                for r in 0..table.len() {
                    for c in 0..table[r].len() {
                        if y==r && x==c {
                            table[r][c] += bonus;
                        } else {
                            table[r][c] -= 1;
                        }
                    }
                }
            }
        }
    }


    print_table(&multibyte_hiragana, &table);

    write_table(&multibyte_hiragana, &table);

    let mut rng = thread_rng();
    let y = rng.gen_range(0, multibyte_hiragana.len());
    if let Some(x) = get_x_by_y(&table, y) {
        println!("{}{}", multibyte_hiragana[x], multibyte_hiragana[y]);
    } else {
        println!("該当なし");
    }
}

fn get_x_by_y(table:&Vec<Vec<i64>>, index:usize) -> Option<usize> {
    let is_max = |value, relay| -> bool {
        relay<value
    };

    if let Some((hit_index, _value)) = each_cell_type3(&table, index, std::i64::MIN, is_max) {
        Some(hit_index)
    } else {
        None
    }
}

fn index_of(ch1:&char, multibyte_hiragana:&Vec<char>, multibyte_katakana:&Vec<char>) -> Option<usize> {
    match multibyte_hiragana.iter().position(|&ch2| ch2==*ch1) {
        Some(x) => return Some(x),
        None => match multibyte_katakana.iter().position(|&ch3| ch3==*ch1) {
            Some(y) => return Some(y),
            None => None
        }
    }
}

fn get_input_2_kana_vec(multibyte_hiragana:&Vec<char>, multibyte_katakana:&Vec<char>) -> Vec<(char,char)> {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("info Failed: stdin.read_line.");

        // Excludes trailing newlines.
        line = line.trim().parse().expect("info Failed: stdin parse.");

        let two_kanas_vec = get_2_kana_vec(&multibyte_hiragana, &multibyte_katakana, &line);

        if 0 < two_kanas_vec.len() {
            return two_kanas_vec;
        }

        println!("Fail. Please input word of 2 hiraganas or more: {}", line);
    }
}

fn get_2_kana_vec(multibyte_hiragana:&Vec<char>, multibyte_katakana:&Vec<char>, contents:&str) -> Vec<(char,char)> {
    let multibyte_line: Vec<char> = contents.chars().collect();

    let mut two_kanas_vec = Vec::new();
    // あれば前の文字
    let mut prev : Option<char> = None;
    // 2文字目から。
    for curr in multibyte_line {
        if multibyte_hiragana.contains(&curr) || multibyte_katakana.contains(&curr) {
            match prev {
                Some(prev_ch) => {
                    two_kanas_vec.push((prev_ch,curr));
                }
                None => {
                }
            }

            prev = Some(curr);
        } else {
            prev = None;
        }
    }

    two_kanas_vec
}

fn read_text() -> Option<String> {
    match fs::read_to_string("input.txt") {
        Ok(contents) => Some(contents),
        Err(_x) => None
    }
}

fn read_table(multibyte_hiragana:&Vec<char>) -> Vec<Vec<i64>>{
    let mut table: Vec<Vec<i64>> = vec![vec![0; multibyte_hiragana.len()]; multibyte_hiragana.len()];

    match csv::Reader::from_path("table.csv") {
        Ok(mut rdr) => {
            for (r, result) in rdr.deserialize().enumerate() {
                let mut record: Vec<String> = result.unwrap();

                // [0]列目のヘッダーを無視。
                record = record[1..].to_vec();

                // println!("Record: {:?}", record);
                let record_i : Vec<i64> = record.iter().map(|s|s.parse().unwrap()).collect();
                table[r] = record_i;
            }
        }
        Err(_x) => {
            // Ignored.
        }
    }

    table
}

fn write_table(multibyte_hiragana:&Vec<char>, table:&Vec<Vec<i64>>){
    let mut header : Vec<&char> = Vec::new();
    header.push(&'　');
    header.extend(multibyte_hiragana);
    let mut wtr = csv::Writer::from_path("table.csv").unwrap();

    match wtr.write_record(
        header.iter().map(|v|v.to_string())
    ) {
        Ok(_x) => {            
        }
        Err(x) => {
            panic!("{}", x);
        }
    }

    for (r, row) in table.iter().enumerate() {
        let mut row2 : Vec<String> = Vec::new();

        // Column header.
        row2.push(multibyte_hiragana[r].to_string());

        // Data. Changes type.
        let vec_of_str : Vec<String> = (*row).iter().map(|num|num.to_string()).collect();
        row2.extend(vec_of_str);

        match wtr.write_record(row2) {
            Ok(_x) => {            
            }
            Err(x) => {
                panic!("{}", x);
            }                
        }
    }

    match wtr.flush() {
        Ok(_x) => {            
        }
        Err(x) => {
            panic!("{}", x);
        }
    }
}


fn print_table(multibyte_hiragana:&Vec<char>, table:&Vec<Vec<i64>>){
    print!("　");
    for ch in multibyte_hiragana.iter() {
        print!(" {}", ch);
    }
    println!();

    each_cell_type1(
        &table,
        |r|print!("{}", multibyte_hiragana[r]),
        |_r, cell|print!("{:>3}", cell),
        |_r|println!());
}


fn each_cell_type1<F, G, H>(table:&Vec<Vec<i64>>, before_row:F, get_cell:G, after_row:H)
    where
    F : Fn(usize),
    G : Fn(usize, i64),
    H : Fn(usize) {

    for (r, row) in table.iter().enumerate() {
        before_row(r);

        for column in row.iter() {
            get_cell(r, *column);
        }

        after_row(r);
    }
}

fn each_cell_type3<F>(table:&Vec<Vec<i64>>, index:usize, init_value:i64, callback:F) -> Option<(usize, i64)>
    where F : Fn(i64, i64) -> bool {

    let mut hit : Option<(usize, i64)> = None;
    let mut relay = init_value;
    for column in table[index].iter() {
        if callback(*column, relay) {
            relay = *column;
            hit = Some((index, *column));
        }
    }

    hit
}
