fn main() {
    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s2);
}