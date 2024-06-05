use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ANCHOR: fast
        let fast = async {
            trpl::sleep(Duration::from_secs(1)).await;
            "I finished!"
        };
        // ANCHOR_END: fast

        match trpl::timeout(Duration::from_secs(2), fast).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}
