// ANCHOR: all
fn main() {
    hello("async");
}

async fn hello(name: &str) {
    let greeting = format!("Hello, {name}!");
    println!("{greeting}");
}
// ANCHOR_END: all
