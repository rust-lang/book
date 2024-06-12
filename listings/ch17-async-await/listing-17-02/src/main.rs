// ANCHOR: main
fn main() {
    hello_async().await;
}
// ANCHOR_END: main

async fn hello_async() {
    println!("Hello, async!");
}
