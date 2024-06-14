use std::{future::Future, time::Duration};

fn main() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        // Here we will actually use the new `timeout` with `slow`.
    });
}

// ANCHOR: declaration
async fn timeout<F: Future>(
    max_time: Duration,
    future: F,
) -> Result<F::Output, Duration> {
    // ANCHOR_END: declaration
    unimplemented!()
}
