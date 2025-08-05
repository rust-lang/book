extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: here
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
        // ANCHOR_END: here
    });
}
