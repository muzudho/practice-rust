use std::io;
use std::io::prelude::*;

trait Fruit {
    fn get_size(&self) -> u64;
}

struct Apple {
    size: u64,
}

struct Pine {
    size: u64,
}

impl Fruit for Apple {
    fn get_size(&self) -> u64 {
        println!("apple");
        self.size
    }
}

impl Fruit for Pine {
    fn get_size(&self) -> u64 {
        println!("pine");
        self.size
    }
}

fn main() {
    println!("1:Apple? 2:Pine?");
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let input = input.trim();

    let apple = Apple { size: 1 };
    let pine = Pine { size: 2 };

    let fruit: &dyn Fruit = match input.as_ref() {
        "1" => &apple as &dyn Fruit,
        "2" => &pine as &dyn Fruit,
        _ => panic!("error"),
    };
    println!("size: {}", fruit.get_size());
}
