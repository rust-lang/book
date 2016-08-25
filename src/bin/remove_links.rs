extern crate regex;

use std::io;
use std::io::{Read, Write};
use regex::{Regex, Captures};

fn main () {
    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        panic!(e);
    }

    //let refs = Vec::new();

    let re = Regex::new(r"\[([^\]]+)\](?:(?:\[([^\]]+)\])|(?:\([^\)]+\)))<!-- ignore -->").unwrap();
    let out = re.replace_all(&buffer, |caps: &Captures| {
        // if let Some(ref) = caps.at(2) {
        //     refs.push(ref.to_owned());
        // }

        caps.at(1).unwrap().to_owned();
    });

    write!(io::stdout(), "{}", out).unwrap();

    //println!("refs: {:?}", refs);
}