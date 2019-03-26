struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// ANCHOR: here
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// ANCHOR_END: here

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
