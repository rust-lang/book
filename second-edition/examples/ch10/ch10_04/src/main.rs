#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
/*
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}", integer);
    println!("{:?}", float);

    */

    let wont_work = Point { x: 5., y: 4.0 };
    println!("{:?}", wont_work);
}
