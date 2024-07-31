trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// ANCHOR: here
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
// ANCHOR_END: here
