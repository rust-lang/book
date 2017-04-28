use std::io;
use std::io::{Read, Write};

fn main () {
    let mut is_in_inline_code = false;

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        panic!(e);
    }

    for line in buffer.lines() {
        // only checks for " in html tag on a single line, otherwise lots of formatting breaks.
        let mut is_in_html_tag = false;
        let mut modified_line = &mut String::new();
        let mut previous_char = std::char::REPLACEMENT_CHARACTER;
        let mut chars_in_line = line.chars();

        while let Some(possible_match) = chars_in_line.next() {
            let char_to_push;
            // check if inside inline code
            if possible_match == '`' {
                is_in_inline_code = !is_in_inline_code;
            }
            // check if inside html tag
            if possible_match == '<' && !is_in_inline_code {
                is_in_html_tag = true;
            }
            if possible_match == '>' {
                is_in_html_tag = false;
            }

            // replace with right/left apostrophe/quote
            if possible_match == '\'' && !is_in_inline_code && !is_in_html_tag {
                if previous_char.is_alphanumeric() || previous_char == '‘' {
                    char_to_push = '’';
                }
                else {
                    char_to_push = '‘';
                }
            }
            else if possible_match == '"'  && !is_in_inline_code && !is_in_html_tag {
                if previous_char.is_alphanumeric() || previous_char == '“' {
                    char_to_push = '”';
                }
                else {
                    char_to_push = '“';
                }
            }
            // leave untouched
            else {
                char_to_push = possible_match;
            }
            modified_line.push(char_to_push);
            previous_char = char_to_push;
        }
        write!(io::stdout(), "{}\n", modified_line).unwrap();
    }
}

