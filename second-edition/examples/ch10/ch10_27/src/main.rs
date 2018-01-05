//fn first_word(s: &str) -> &str {
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5.
	println!("{}",word);

}
