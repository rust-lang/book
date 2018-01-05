fn main() {
    let s = String::from("hello");

    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);
}
