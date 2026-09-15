fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest_with_an_announcement(
        &string1,
        &string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {result}");
}

// ANCHOR: here
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
// ANCHOR_END: here
