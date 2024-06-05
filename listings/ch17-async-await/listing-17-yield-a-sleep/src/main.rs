use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ANCHOR: here
        let pause = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            for _ in 1..5 {
                println!("'a' made progress. Yielding control.");
                trpl::sleep(pause).await;
            }
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            for _ in 1..5 {
                println!("'b' made progress. Yielding control.");
                trpl::sleep(pause).await;
            }
            println!("'b' finished.");
        };
        // ANCHOR_end: here

        trpl::race(a, b).await;
    });
}
