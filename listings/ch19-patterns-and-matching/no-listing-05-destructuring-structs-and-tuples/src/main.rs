fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    // ANCHOR: here
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    // ANCHOR_END: here
}
