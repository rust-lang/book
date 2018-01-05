struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let p1 = Point { x: 5, y: 10 };


    println!("p1.x() = {}", p1.x());

    println!("distance from origin =  {}", p1.distance_from_origin());
}
