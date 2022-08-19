#[derive(Debug)]
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

// ANCHOR: here
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter => 5,
        Coin::Dime => 10,
        Coin::Nickel(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
// ANCHOR_END: here

fn main() {
    value_in_cents(Coin::Nickel(UsState::Alaska));
}
