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

    let lines: Vec<_> = input.lines().flat_map(|line| {
        // Remove our figure and caption markup
        if line == "<figure>" || line == "<figcaption>" || line == "</figcaption>" || line == "</figure>" {
            None
        // Remove our syntax highlighting and rustdoc markers
        } else if line.starts_with("```") {
            Some(String::from("```"))
        // Remove the span around filenames
        } else {
            let result = filename_regex.replace_all(line, |caps: &Captures| {
                caps.at(1).unwrap().to_owned()
            });
            Some(result)
        }
    }).collect();
    lines.join("\n")
}
