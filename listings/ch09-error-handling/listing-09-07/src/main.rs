// ANCHOR: here
use std::fs::File;
use std::io;
use std::fs;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let filename = "hello.txt";
    let mut f = File::open(filename)?;
    fs::read_to_string(filename)
}
// ANCHOR_END: here

fn main() {
    let username = read_username_from_file().expect("Unable to get username");
}
