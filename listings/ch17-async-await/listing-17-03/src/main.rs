// ANCHOR: main
fn main() {
    trpl::block_on(async {
        hello("async").await;
    });
}
// ANCHOR_END: main

async fn hello(name: &str) {
    let greeting = format!("Hello, {name}!");
    println!("{greeting}");
}
