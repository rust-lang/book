fn main() {
    // ANCHOR: here
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
    // ANCHOR_END: here
}
