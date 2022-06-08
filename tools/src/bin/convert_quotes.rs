use std::io;
use std::io::Read;

fn main() {
    let mut is_in_code_block = false;
    let mut is_in_inline_code = false;
    let mut is_in_html_tag = false;

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        panic!("{}", e);
    }

    for line in buffer.lines() {
        if line.is_empty() {
            is_in_inline_code = false;
        }
        if line.starts_with("```") {
            is_in_code_block = !is_in_code_block;
        }
        if is_in_code_block {
            is_in_inline_code = false;
            is_in_html_tag = false;
            println!("{}", line);
        } else {
            let modified_line = &mut String::new();
            let mut previous_char = std::char::REPLACEMENT_CHARACTER;
            let chars_in_line = line.chars();

            for possible_match in chars_in_line {
                // Check if inside inline code.
                if possible_match == '`' {
                    is_in_inline_code = !is_in_inline_code;
                }
                // Check if inside HTML tag.
                if possible_match == '<' && !is_in_inline_code {
                    is_in_html_tag = true;
                }
                if possible_match == '>' && !is_in_inline_code {
                    is_in_html_tag = false;
                }

                // Replace with right/left apostrophe/quote.
                let char_to_push = if possible_match == '\''
                    && !is_in_inline_code
                    && !is_in_html_tag
                {
                    if (previous_char != std::char::REPLACEMENT_CHARACTER
                        && !previous_char.is_whitespace())
                        || previous_char == '‘'
                    {
                        '’'
                    } else {
                        '‘'
                    }
                } else if possible_match == '"'
                    && !is_in_inline_code
                    && !is_in_html_tag
                {
                    if (previous_char != std::char::REPLACEMENT_CHARACTER
                        && !previous_char.is_whitespace())
                        || previous_char == '“'
                    {
                        '”'
                    } else {
                        '“'
                    }
                } else {
                    // Leave untouched.
                    possible_match
                };
                modified_line.push(char_to_push);
                previous_char = char_to_push;
            }
            println!("{}", modified_line);
        }
    }
}
