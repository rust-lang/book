fn main() {
    // ANCHOR: here
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    dbg!(s);
    // ANCHOR_END: here
}
