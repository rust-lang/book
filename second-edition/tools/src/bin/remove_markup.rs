// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate regex;
use std::io;
use std::io::{Read, Write};
use regex::{Regex, Captures};

fn main() {
    write_md(remove_markup(read_md()));
}

fn read_md() -> String {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => buffer,
        Err(error) => panic!(error),
    }
}

fn write_md(output: String) {
    write!(io::stdout(), "{}", output).unwrap();
}

fn remove_markup(input: String) -> String {
    let filename_regex = Regex::new(r#"\A<span class="filename">(.*)</span>\z"#).unwrap();
    // Captions sometimes take up multiple lines
    let caption_start_regex = Regex::new(r#"\A<span class="caption">(.*)\z"#).unwrap();
    let caption_end_regex = Regex::new(r#"(.*)</span>\z"#).unwrap();
    let regexen = vec![filename_regex, caption_start_regex, caption_end_regex];

    let lines: Vec<_> = input.lines().flat_map(|line| {
        // Remove our figure and caption markup
        if line == "<figure>" ||
            line == "<figcaption>" ||
            line == "</figcaption>" ||
            line == "</figure>"
        {
            None
        // Remove our syntax highlighting and rustdoc markers
        } else if line.starts_with("```") {
            Some(String::from("```"))
        // Remove the span around filenames and captions
        } else {
            let result = regexen.iter().fold(line.to_string(), |result, regex| {
                regex.replace_all(&result, |caps: &Captures| {
                    caps.at(1).unwrap().to_owned()
                })
            });
            Some(result)
        }
    }).collect();
    lines.join("\n")
}
