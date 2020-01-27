// ANCHOR: here
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
// ANCHOR_END: here

fn main() {
    let username = read_username_from_file().expect("Unable to get username");
}
