fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{}", r1);
    println!("{}", r2);
    println!("{}", r3);
}