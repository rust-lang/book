use std::io;
use std::io::{Read, Write};

fn replace_straights_by_curlys(line : &str, modified_line : &mut std::string::String, mut is_in_inline_code : bool){
    // right curly if in middle or end of a word. left curly if at beginning.
    let mut line_chars = line.chars();
    let mut previous_char = '⌨'; // default non-alphanumeric char

    while let Some(possible_match) = line_chars.next() {
        // check if inside a codeblock
        if possible_match == '`' {
            is_in_inline_code = !is_in_inline_code;
        }

        if possible_match == '\'' && !is_in_inline_code {
            if previous_char.is_alphanumeric() || previous_char == '‘' {
                // replace with right apostrophe
                modified_line.push('’');
                previous_char = '’';
            }
            else {
                // replace with left apostrophe
                modified_line.push('‘');
                previous_char = '‘';
            }
        }
        else if possible_match == '"'  && !is_in_inline_code {
            if previous_char.is_alphanumeric() || previous_char == '“' {
                // replace with right quote
                modified_line.push('”');
                previous_char = '”';
            }
            else {
                // replace with left quote
                modified_line.push('“');
                previous_char = '“';
            }
        }
        else {
            // leave untouched
            modified_line.push(possible_match);
            previous_char = possible_match;
        }
    }
}

fn main () {
    let mut is_in_code_block = false;
    let is_in_inline_code = false;

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        panic!(e);
    }

    let lines = buffer.lines();
    for line in lines {
        if line.starts_with("```") {
            is_in_code_block = !is_in_code_block;
        }
        if !is_in_code_block {
            let mut modified_line = &mut String::new();
            replace_straights_by_curlys(line, modified_line, is_in_inline_code );
            write!(io::stdout(), "{}\n", modified_line).unwrap();
        } else {
            write!(io::stdout(), "{}\n", line).unwrap();
        }
    }
}


