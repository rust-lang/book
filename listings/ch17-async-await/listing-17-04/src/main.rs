extern crate trpl; // required for mdbook test

// ANCHOR: main
fn main() {
    trpl::run(async {
        hello("async").await;
    });
}
// ANCHOR_END: main

async fn hello(name: &str) {
    let greeting = format!("Hello, {name}!");
    println!("{greeting}");
}
