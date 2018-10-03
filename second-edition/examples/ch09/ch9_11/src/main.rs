use std::net::IpAddr;

fn main() {
    
    let home = "127.0.0.1".parse::<IpAddr>().unwrap();
    println!("{}", home);
}