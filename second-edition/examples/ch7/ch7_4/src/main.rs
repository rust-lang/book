#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    println!("{:?}",red);
    println!("{:?}",yellow);
    println!("{:?}",green);
}