#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ANCHOR: here
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
// ANCHOR_END: here
