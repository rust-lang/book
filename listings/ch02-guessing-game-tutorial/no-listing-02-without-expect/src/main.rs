use std::io;

fn main() {
    println!("¡Adivina el número!");

    println!("Por favor, introduce tu predicción.");

    let mut prediccion = String::new();

    io::stdin().read_line(&mut prediccion);

    println!("You guessed: {}", prediccion);
}
