extern crate regex;

use regex::{Captures, Regex};
use std::io;
use std::io::{Read, Write};

fn main() {
    write_md(remove_markup(read_md()));
}

fn read_md() -> String {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => buffer,
        Err(error) => panic!("{}", error),
    }
}

fn write_md(output: String) {
    write!(io::stdout(), "{}", output).unwrap();
}

fn remove_markup(input: String) -> String {
    let filename_regex =
        Regex::new(r#"\A<span class="filename">(.*)</span>\z"#).unwrap();
    // Captions sometimes take up multiple lines.
    let caption_start_regex =
        Regex::new(r#"\A<span class="caption">(.*)\z"#).unwrap();
    let caption_end_regex = Regex::new(r#"(.*)</span>\z"#).unwrap();
    let regexen = vec![filename_regex, caption_start_regex, caption_end_regex];

    let lines: Vec<_> = input
        .lines()
        .flat_map(|line| {
            // Remove our syntax highlighting and rustdoc markers.
            if line.starts_with("```") {
                Some(String::from("```"))
            // Remove the span around filenames and captions.
            } else {
                let result =
                    regexen.iter().fold(line.to_string(), |result, regex| {
                        regex
                            .replace_all(&result, |caps: &Captures<'_>| {
                                caps.get(1).unwrap().as_str().to_string()
                            })
                            .to_string()
                    });
                Some(result)
            }
        })
        .collect();
    lines.join("\n")
}
