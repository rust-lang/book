use std::fs::File;

fn main() {
    // ANCHOR: here
    let f: u32 = File::open("hello.txt");
    // ANCHOR_END: here
}
