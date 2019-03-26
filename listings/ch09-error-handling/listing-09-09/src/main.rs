// ANCHOR: here
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
// ANCHOR_END: here

fn main() {
    let username = read_username_from_file().expect("Unable to get username");
}
