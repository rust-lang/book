use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    assert_eq!(
        Millimeters ( 2 ) + Millimeters ( 2 ),
        Millimeters ( 4 )
    );
    assert_eq!(
        Millimeters ( 500 ) + Meters ( 1 ),
        Millimeters ( 1500 )
    );
}
