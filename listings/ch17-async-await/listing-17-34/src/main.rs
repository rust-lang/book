use std::{future::Future, time::Duration};

// ANCHOR: timeout
use trpl::Either;
// ANCHOR_END: timeout

fn main() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        // Here we will actually use the new `timeout` with `slow`.
    });
}

// Note for maintainers: the extra space after the `ANCHOR` is intentional: it
// makes this render more nicely!
// ANCHOR: timeout

async fn timeout<F: Future>(
    max_time: Duration,
    future: F,
) -> Result<F::Output, Duration> {
    match trpl::race(future, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
// ANCHOR_END: timeout
