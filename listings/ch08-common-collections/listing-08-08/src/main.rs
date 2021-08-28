fn main() {
    // ANCHOR: here
    let v = vec![100, 32, 57];
    for i in &v {
        dbg!(i);
    }
    // ANCHOR_END: here
}
