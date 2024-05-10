use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // ANCHOR: here
        let a = async {
            println!("'a' started.");
            slow("a", 300);
            trpl::yield_now().await;
            slow("a", 100);
            trpl::yield_now().await;
            slow("a", 200);
            trpl::yield_now().await;
            slow("a", 900);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 750);
            trpl::yield_now().await;
            slow("b", 100);
            trpl::yield_now().await;
            slow("b", 150);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            slow("b", 150);
            trpl::yield_now().await;
            println!("'b' finished.");
        };
        // ANCHOR_end: here

        trpl::race(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
