// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("¡Adivina el número!");

    println!("Por favor, introduce tu predicción.");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut prediccion = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
        .read_line(&mut prediccion)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("Lectura de línea fallida");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("Predijiste: {}", prediccion);
    // ANCHOR_END: print_guess
}
// ANCHOR: all
