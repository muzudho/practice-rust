// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first  element of the slice: {}", slice[0]);
    println!("second element of the slice: {}", slice[1]);
    println!("third  element of the slice: {}", slice[2]);
    println!("fourth element of the slice: {}", slice[3]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    //        0  1  2  3  4  5  6
    let xs = [3, 4, 5, 6, 7, 8 ,9];

    // 終端を含まない
    analyze_slice(&xs[1 .. 5]);
}
