fn main() {
    // ANCHOR: here
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        dbg!(top);
    }
    // ANCHOR_END: here
}
