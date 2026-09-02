// ANCHOR: here
fn first_word(s: &String) -> &str {

    for (i, ch) in s.char_indices() {
        if ch.is_whitespace() {
            return &s[..i];
        }
    }

    &s[..]
}
// ANCHOR_END: here

fn main() {}
