extern crate trpl; // required for mdbook test

use std::{future::Future, time::Duration};

fn main() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

// ANCHOR: declaration
async fn timeout<F: Future>(
    future: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    // Here is where our implementation will go!
}
// ANCHOR_END: declaration
