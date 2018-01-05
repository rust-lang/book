fn main() {
    let x = 5;

    match x {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }
    
    for x in 1..7 {
        match x {
            1...5 => println!("`for` one through five"),
            _ => println!("something else"),
        }
    }
}
