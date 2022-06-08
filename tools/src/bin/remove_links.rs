extern crate regex;

use regex::{Captures, Regex};
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        panic!("{}", e);
    }

    let mut refs = HashSet::new();

    // Capture all links and link references.
    let regex =
        r"\[([^\]]+)\](?:(?:\[([^\]]+)\])|(?:\([^\)]+\)))(?i)<!--\signore\s-->";
    let link_regex = Regex::new(regex).unwrap();
    let first_pass = link_regex.replace_all(&buffer, |caps: &Captures<'_>| {
        // Save the link reference we want to delete.
        if let Some(reference) = caps.get(2) {
            refs.insert(reference.as_str().to_string());
        }

        // Put the link title back.
        caps.get(1).unwrap().as_str().to_string()
    });

    // Search for the references we need to delete.
    let ref_regex = Regex::new(r"(?m)^\[([^\]]+)\]:\s.*\n").unwrap();
    let out = ref_regex.replace_all(&first_pass, |caps: &Captures<'_>| {
        let capture = caps.get(1).unwrap().to_owned();

        // Check if we've marked this reference for deletion ...
        if refs.contains(capture.as_str()) {
            return "".to_string();
        }

        // ... else we put back everything we captured.
        caps.get(0).unwrap().as_str().to_string()
    });

    print!("{}", out);
}
