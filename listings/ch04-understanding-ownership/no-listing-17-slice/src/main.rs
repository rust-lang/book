fn main() {
    // ANCHOR: here
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    // ANCHOR_END: here
}
