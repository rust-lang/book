// ANCHOR: here
fn first_word(s: &String) -> usize {
    // ANCHOR: as_bytes
    let bytes = s.as_bytes();
    // ANCHOR_END: as_bytes

    // ANCHOR: iter
    for (i, &item) in bytes.iter().enumerate() {
        // ANCHOR_END: iter
        // ANCHOR: inside_for
        if item == b' ' {
            return i;
        }
    }

    s.len()
    // ANCHOR_END: inside_for
}
// ANCHOR_END: here

fn main() {}
