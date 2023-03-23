#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

const GOLDEN_RATIO: f32 = 1.618;

// ANCHOR: here
impl Rectangle {
    fn golden_rectangle(width: f32) -> Self {
        Self {
            width,
            height: GOLDEN_RATIO * width,
        }
    }
}
// ANCHOR_END: here

fn main() {
    let sq = Rectangle::golden_rectangle(3.0);
}
