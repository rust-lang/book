// ANCHOR: all
use std::io;

// ANCHOR: ch07-04
use rand::random_range;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Guess the number!");

    // ANCHOR: ch07-04
    // let secret_number = rand::thread_rng().gen_range(1..=100); is replaced by:
    let secret_number = random_range(1..=100);
    // ANCHOR_END: ch07-04

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all