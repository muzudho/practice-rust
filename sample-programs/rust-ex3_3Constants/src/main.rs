// Globals are declared outside all other scopes.
static LANGUAGE: &'static str = "Rust";
const  LANGUAGE2: &'static str = "Rust"; // 自習
const  THRESHOLD: i32 = 10;
static THRESHOLD2: i32 = 10; // 自習

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("This is {}", LANGUAGE2); // 自習
    println!("The threshold is {}", THRESHOLD);
    println!("The threshold is {}", THRESHOLD2); // 自習
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    //THRESHOLD = 5;
    // FIXME ^ Comment out this line
}
