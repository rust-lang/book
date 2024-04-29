// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
// ANCHOR_END: here

fn main() {
    let username = read_username_from_file().expect("Unable to get username");
}
