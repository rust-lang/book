fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// ANCHOR: here
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// ANCHOR_END: here
