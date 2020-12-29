fn main() {
    // ANCHOR: here
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    // ANCHOR_END: here
}
