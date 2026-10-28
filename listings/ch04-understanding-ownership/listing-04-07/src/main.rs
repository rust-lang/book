// ANCHOR: here
fn first_word(s: &String) -> usize {
    // ANCHOR: char_indices
    let char_indices = s.char_indices();
    // ANCHOR_END: char_indices
    // ANCHOR: iter
    for (i, ch) in char_indices {
        // ANCHOR_END: iter
        // ANCHOR: inside_for
        if ch.is_whitespace() {
            return i;
        }
    }

    s.len()
    // ANCHOR_END: inside_for
}
// ANCHOR_END: here

fn main() {}
