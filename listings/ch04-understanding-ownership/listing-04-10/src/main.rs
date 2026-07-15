// ANCHOR: here
fn nth_word(s: &str, n: usize) -> &str {
    // The minimum acceptable value for `n` is 1.
    if n == 0 {
        return s;
    }
    let mut n = n;
    let mut word_start_index = 0;
    // This flag is set to `true` when skipping leading spaces or spaces between words
    let mut skipping_spaces = true;
    for (i, ch) in s.char_indices() {
        if skipping_spaces {
            if ch.is_whitespace() {
                continue;
            }
            skipping_spaces = false;
            word_start_index = i;
        } else if ch.is_whitespace() {
            // End of current word
            n -= 1;
            if n == 0 {
                return &s[word_start_index..i];
            }
            // Skip spaces before looking for the next word
            skipping_spaces = true;
        } else if i + 1 == s.len() && n == 1 {
            // We are at end of `s` and are looking for the last word
            return &s[word_start_index..];
        }
    }
    s
}
// ANCHOR_END: here

// ANCHOR: usage
fn main() {
    // Entire word should be returned when looking for 0th word or
    // when looking for a word beyond words in the input
    assert_eq!(nth_word("1 2 3", 0), "1 2 3");
    assert_eq!(nth_word("1 2 3 4", 5), "1 2 3 4");

    // Input word should be returned when no word exists
    assert_eq!(nth_word("", 1), "");
    assert_eq!(nth_word("  ", 1), "  ");

    // Entire word should be returned when there's only 1 word
    assert_eq!(nth_word("s", 1), "s");
    assert_eq!(nth_word("hello", 1), "hello");

    // Correct word should be returned irrespective of number of whitespaces
    assert_eq!(nth_word("hello ", 1), "hello");
    assert_eq!(nth_word(" hello ", 1), "hello");
    assert_eq!(nth_word("\t \nhello", 1), "hello");

    // Correct word should be returned when there are multiple words
    assert_eq!(nth_word("so how are you?", 1), "so");
    assert_eq!(nth_word("so how are you?", 2), "how");
    assert_eq!(nth_word("so how are you?", 3), "are");
    assert_eq!(nth_word("so how are you?", 4), "you?");
    assert_eq!(nth_word(" hello  how   are ", 3), "are");
}
// ANCHOR_END: usage