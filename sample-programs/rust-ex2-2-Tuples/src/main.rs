use std::fmt; // 演習で追加

// 演習2
fn transpose(m: Matrix) -> Matrix{
    Matrix(m.0, m.2, m.1, m.3)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
// 演習で追加
impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 末尾にセミコロンは付けない
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    //println!("{:?}", matrix);
    println!("{}", transpose(matrix)); // 演習で追加

    // 演習2
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
