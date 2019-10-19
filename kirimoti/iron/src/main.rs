//!
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti
//! cargo new iron
//! cd C:\Users\むずでょ\OneDrive\ドキュメント\practice-rust\kirimoti\iron
//! cargo check
//! cargo run
//! 
//! [crates.io](https://crates.io/)
//!     rand
//! 

use rand::prelude::*;

fn main() {
    let mut text1 = "アイウエオ　カキクケコ　サシスセソ　タチツテト　ナニヌネノ　ハヒフヘホ　マミムメモ　ヤユヨ　ワオガギグゲゴ　ザジズゼゾ　ダヂヅデド　バビブベボ　パピプペポ".to_string();
    text1.retain(|c| c != '　');
    let multibyte_text1: Vec<char> = text1.chars().collect();

    let mut text2 = "アイウエオ　カキクケコ　サシスセソ　タチツテト　ナニヌネノ　ハヒフヘホ　マミムメモ　ヤユヨ　ワオンガギグゲゴ　ザジズゼゾ　ダヂヅデド　バビブベボ　パピプペポ　ャュョ　ヴ".to_string();
    text2.retain(|c| c != '　');
    let multibyte_text2: Vec<char> = text2.chars().collect();

    let mut rng = thread_rng();
    let index1 = rng.gen_range(0, multibyte_text1.len());
    let index2 = rng.gen_range(0, multibyte_text2.len());

    println!("Quiz: {}□□{}", multibyte_text1[index1], multibyte_text2[index2]);
}
