fn main() {
    // ANCHOR: here
    let config_max = Some(u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // ANCHOR_END: here
}
