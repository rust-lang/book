fn main() {
    let some_u8_value = Some(0u8);
    // ANCHOR: here
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // ANCHOR_END: here
}
