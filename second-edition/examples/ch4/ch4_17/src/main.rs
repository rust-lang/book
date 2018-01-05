fn main() {
    let s = String::from("hello");
    println!("{}", s);
    let len = s.len();
    println!("a length of the string = {}", s);
    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);
}
