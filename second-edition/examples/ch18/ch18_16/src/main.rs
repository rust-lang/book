fn main() {
    let x = Some(5);

    match x {
        Some(_) => println!("got a Some and I don't care what's inside"),
        None => (),
    }
}
