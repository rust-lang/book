// ANCHOR: here
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Quarter,
    Dime,
    Nickel(UsState),
}
// ANCHOR_END: here

fn main() {}
