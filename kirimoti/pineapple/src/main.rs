//!
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti
//! cargo new pineapple
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti\pineapple
//! cargo check
//! cargo run
//! 
//! [crates.io](https://crates.io/)
//! 

fn main() {
    /* // Making of -.
    let mut hiragana = "あいうえお　かきくけこ　さしすせそ　たちつてと　なにぬねの　はひふへほ　まみむめも　やゆよ　わおん
        がきぐげご　ざじずぜぞ　だぢづでど　ばびぶべぼ　ぱぴぷぺぽ　ゃゅょ　ヴ
        ぁぃぅぇぉ".to_string();
    hiragana.retain(|c| c != ' ' && c != '　' && c != '\r' && c != '\n');
    println!("let hiragana = \"{}\".to_string();",hiragana);
    */
    let hiragana = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよわおんがきぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽゃゅょヴぁぃぅぇぉ".to_string();

    /* // Making of -.
    let mut katakana = "アイウエオ　カキクケコ　サシスセソ　タチツテト　ナニヌネノ　ハヒフヘホ　マミムメモ　ヤユヨ　ワオン
        ガギグゲゴ　ザジズゼゾ　ダヂヅデド　バビブベボ　パピプペポ　ャュョ　ヴ
        ァィゥェォ".to_string();
    katakana.retain(|c| c != ' ' && c != '　' && c != '\r' && c != '\n');
    println!("let katakana = \"{}\".to_string();",katakana);
    */
    let katakana = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨワオンガギグゲゴザジズゼゾダヂヅデドバビブベボパピプペポャュョヴァィゥェォ".to_string();

    let multibyte_hiragana: Vec<char> = hiragana.chars().collect();
    let multibyte_katakana: Vec<char> = katakana.chars().collect();
    let mut table: Vec<Vec<i64>> = vec![vec![0; multibyte_hiragana.len()]; multibyte_hiragana.len()];

    let bonus = multibyte_hiragana.len() as i64;
    let two_kanas_vec = get_input_2_kana_vec(&multibyte_hiragana, &multibyte_katakana);
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

        let multibyte_line: Vec<char> = line.chars().collect();

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

        if 0 < two_kanas_vec.len() {
            return two_kanas_vec;
        }

        println!("Fail. Please input word of 2 hiraganas or more: {}", line);
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
