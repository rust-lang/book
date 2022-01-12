// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("Tuttuğum sayıyı tahmin edin!");

    println!("Lütfen tahmininizi giriniz.");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut tahmin = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
    	.read_line(&mut tahmin)
      // ANCHOR_END: read
      // ANCHOR: expect
    	.expect("Veri okuma hatası!");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("Tahmin ettiğiniz sayı: {}", tahmin);
    // ANCHOR_END: print_guess
}
// ANCHOR: all 
