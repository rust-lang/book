use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // ANCHOR: slow-futures
        let a = async {
            println!("'a' started.");
            slow("a", 300);
            slow("a", 100);
            slow("a", 200);
            slow("a", 900);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 750);
            slow("b", 100);
            slow("b", 150);
            slow("b", 350);
            slow("b", 150);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
        // ANCHOR_END: slow-futures
    });
}

// ANCHOR: slow
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
// ANCHOR_END: slow
