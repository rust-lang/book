fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        map.entry(word)
            .and_modify(|existing_value| *existing_value += 1)
            .or_insert(1);
    }

    println!("{:?}", map);
    // ANCHOR_END: here
}
