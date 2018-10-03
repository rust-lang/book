struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: p1.x, y: p1.y };

    println!("p1.x() = {}", p1.x());
    println!("p2.x = {}", p2.x());
}
