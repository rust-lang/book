fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
                       // ANCHOR_END: here
}
