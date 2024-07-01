use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // ANCHOR: here
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 300);
            trpl::sleep(one_ms).await;
            slow("a", 100);
            trpl::sleep(one_ms).await;
            slow("a", 200);
            trpl::sleep(one_ms).await;
            slow("a", 900);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 750);
            trpl::sleep(one_ms).await;
            slow("b", 100);
            trpl::sleep(one_ms).await;
            slow("b", 150);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            slow("b", 150);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };
        // ANCHOR_END: here

        trpl::race(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
