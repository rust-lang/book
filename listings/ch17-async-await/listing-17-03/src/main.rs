// ANCHOR: main
fn main() {
    trpl::block_on(hello_async());
}
// ANCHOR_END: main

async fn hello_async() {
    println!("Hello, async!");
}
