fn main() {
    let s = String::from("hello");

    let len = s.len();
    println!("sting length = {}", len);
    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);
}
