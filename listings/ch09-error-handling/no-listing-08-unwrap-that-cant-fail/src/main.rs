fn main() {
    // ANCHOR: here
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    // ANCHOR_END: here
}
