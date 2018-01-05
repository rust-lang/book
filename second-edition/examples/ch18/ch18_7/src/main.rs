fn main() {
    let x = Some(3);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        //Some(y) => println!("Matched, y = {:?}", y),
        Some(z) => println!("Matched, z = {:?}", z),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
