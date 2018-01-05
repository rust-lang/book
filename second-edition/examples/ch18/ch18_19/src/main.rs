struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}
