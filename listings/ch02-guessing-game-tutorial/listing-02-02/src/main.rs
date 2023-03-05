use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("String variable `guess` should be set by `read_line`");

    println!("You guessed: {guess}");
}
