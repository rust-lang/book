// ANCHOR: here
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    // ANCHOR_END: here
    tx.send(()).unwrap();
    // ANCHOR: here
}
// ANCHOR_END: here
