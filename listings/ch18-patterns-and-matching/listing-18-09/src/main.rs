fn main() {
    let some_option_value: Option<i32> = None;
    // ANCHOR: here
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
    // ANCHOR_END: here
}
