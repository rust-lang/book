fn main() {
    let s = String::from("hello");

    let len = s.len();
    println!("string length = {}", len);
    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);

    //why this code is worked?!
    let slice = &s[5..];
    println!("{}", slice);
}
