fn main() {
    // ANCHOR: here
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    // ANCHOR_END: here
}
