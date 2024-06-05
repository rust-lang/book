fn main() {
    trpl::block_on(async {
        // ANCHOR: here
        let a = async {
            println!("'a' started.");
            for _ in 1..5 {
                println!("'a' made progress. Yielding control.");
                trpl::yield_now().await;
            }
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            for _ in 1..5 {
                println!("'b' made progress. Yielding control.");
                trpl::yield_now().await;
            }
            println!("'b' finished.");
        };
        // ANCHOR_end: here

        trpl::race(a, b).await;
    });
}
