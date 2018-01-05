#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;

    println!("{:?}",red);
    println!("{:?}",yellow);
    println!("{:?}",green);
}