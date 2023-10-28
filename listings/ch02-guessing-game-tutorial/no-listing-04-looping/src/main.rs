use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // ANCHOR: here
    // --snip--

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("The secret number is: {secret_number}");
        
        println!("Please input your guess.");

        // --snip--

        // ANCHOR_END: here

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        // ANCHOR: here
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
// ANCHOR_END: here
