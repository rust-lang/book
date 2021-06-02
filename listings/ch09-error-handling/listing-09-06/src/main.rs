// ANCHOR: here
use std::fs::File;
use std::fs;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let filename = "hello.txt";
    let f = File::open(filename);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match fs::read_to_string(filename) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// ANCHOR_END: here

fn main() {
    let username = read_username_from_file().expect("Unable to get username");
}
