use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ANCHOR: here
        let limit = Duration::from_secs(2);
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match trpl::timeout(limit, slow).await {
            Ok(message) => println!("Succeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
        // ANCHOR_END: here
    });
}
