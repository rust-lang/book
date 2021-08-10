fn main() {
    // ANCHOR: here
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // ANCHOR_END: here
}
