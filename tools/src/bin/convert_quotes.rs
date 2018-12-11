// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use std::io::{Read, Write};

fn main() {
    let mut is_in_code_block = false;
    let mut is_in_inline_code = false;
    let mut is_in_html_tag = false;

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        panic!(e);
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
            write!(io::stdout(), "{}\n", line).unwrap();
        } else {
            let mut modified_line = &mut String::new();
            let mut previous_char = std::char::REPLACEMENT_CHARACTER;
            let mut chars_in_line = line.chars();

            while let Some(possible_match) = chars_in_line.next() {
                // check if inside inline code
                if possible_match == '`' {
                    is_in_inline_code = !is_in_inline_code;
                }
                // check if inside html tag
                if possible_match == '<' && !is_in_inline_code {
                    is_in_html_tag = true;
                }
                if possible_match == '>' && !is_in_inline_code {
                    is_in_html_tag = false;
                }

                // replace with right/left apostrophe/quote
                let char_to_push =
                    if possible_match == '\'' && !is_in_inline_code && !is_in_html_tag {
                        if (previous_char != std::char::REPLACEMENT_CHARACTER &&
                                !previous_char.is_whitespace()) ||
                            previous_char == '‘'
                        {
                            '’'
                        } else {
                            '‘'
                        }
                    } else if possible_match == '"' && !is_in_inline_code && !is_in_html_tag {
                        if (previous_char != std::char::REPLACEMENT_CHARACTER &&
                                !previous_char.is_whitespace()) ||
                            previous_char == '“'
                        {
                            '”'
                        } else {
                            '“'
                        }
                    } else {
                        // leave untouched
                        possible_match
                    };
                modified_line.push(char_to_push);
                previous_char = char_to_push;
            }
            write!(io::stdout(), "{}\n", modified_line).unwrap();
        }
    }
}
