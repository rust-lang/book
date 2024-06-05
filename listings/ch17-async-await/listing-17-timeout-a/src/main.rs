use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ANCHOR: slow
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "I finished!"
        };
        // ANCHOR_END: slow

        match trpl::timeout(Duration::from_secs(2), slow).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}
