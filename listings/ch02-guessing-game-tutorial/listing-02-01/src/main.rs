// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("Guess the number!");

    println!("Please input your guess.");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut guess = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
        .read_line(&mut guess)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("Failed to read line");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("You guessed: {}", guess);
    // ANCHOR_END: print_guess
}
// ANCHOR: all
