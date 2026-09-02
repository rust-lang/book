fn first_word(s: &String) -> &str {
    
    for (i, ch) in s.char_indices() {
        if ch.is_whitespace() {
            return &s[..i];
        }
    }

    &s[..]
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {word}");
}
// ANCHOR_END: here
