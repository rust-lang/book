extern crate regex;

use std::io;
use std::io::{Read, Write};
use regex::{Regex, Captures};
use std::collections::HashSet;

fn main () {
    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        panic!(e);
    }

    let mut refs = HashSet::new();

    // Capture all links and link references.
    let regex = r"\[([^\]]+)\](?:(?:\[([^\]]+)\])|(?:\([^\)]+\)))(?i)<!-- ignore -->";
    let link_regex = Regex::new(regex).unwrap();
    let first_pass = link_regex.replace_all(&buffer, |caps: &Captures| {

        // Save the link reference we want to delete.
        if let Some(reference) = caps.at(2) {
            refs.insert(reference.to_owned());
        }

        // Put the link title back.
        caps.at(1).unwrap().to_owned()
    });

    // Search for the references we need to delete.
    let ref_regex = Regex::new(r"\n\[([^\]]+)\]:\s.*\n").unwrap();
    let out = ref_regex.replace_all(&first_pass, |caps: &Captures| {
        let capture = caps.at(1).unwrap().to_owned();

        // Check if we've marked this reference for deletion ...
        if refs.contains(capture.as_str()) {
            return "".to_string();
        }

        // ... else we put back everything we captured.
        caps.at(0).unwrap().to_owned()
    });

    write!(io::stdout(), "{}", out).unwrap();
}
