fn main() {
    // ANCHOR: here
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
    // ANCHOR_END: here
}
