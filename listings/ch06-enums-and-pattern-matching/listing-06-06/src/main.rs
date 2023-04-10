fn main() {
    // ANCHOR: here
    let config_max: Option<u8> = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // ANCHOR_END: here
}
