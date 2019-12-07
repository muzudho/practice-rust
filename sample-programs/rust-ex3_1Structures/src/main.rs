use std::fmt;//演習2

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]//演習2
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]//演習2
struct Rectangle {
    p1: Point,
    p2: Point,
}

// 演習1
fn rect_area(r : Rectangle) -> f32 {
    (r.p2.x - r.p1.x) * (r.p2.y - r.p1.y)
}
// 演習2
fn square(p : Point, side : f32) -> Rectangle {
    Rectangle{
        p1 : Point{..p}, // コピー
        p2 : Point{ x: p.x + side, y: p.y + side }
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;
    // 演習2: コピー
    let p2 = Point{..point};

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point, // ここで point は _rectangle に独り占めされた
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // 演習1
    println!("rect_area: {}", rect_area(_rectangle));
    // 演習2
    // point は _rectangle に独り占めされたので、コピーの p2 を使う
    println!("square: {:?}", square(p2, 3f32));
}
