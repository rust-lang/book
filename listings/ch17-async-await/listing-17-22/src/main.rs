extern crate trpl; // required for mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // We will call `slow` here later
    });
}

// ANCHOR: slow
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
// ANCHOR_END: slow
