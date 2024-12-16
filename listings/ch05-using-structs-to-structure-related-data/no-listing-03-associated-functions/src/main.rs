#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ANCHOR: here
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
// ANCHOR_END: here

fn main() {
    let sq = Rectangle::square(3);
}
