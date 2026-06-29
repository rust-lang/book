fn main() {
    // ANCHOR: here
    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();
    // ANCHOR_END: here
}
