use std::{future::Future, time::Duration};

use trpl::Either;

fn main() {
    trpl::block_on(async {
        let result = retry(10, || {
            timeout(Duration::from_millis(20), async {
                trpl::sleep(Duration::from_millis(25)).await;
                "Finally finished"
            })
        })
        .await;

        match result {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(errors) => {
                println!(
                    "Failed after {} attempts. Errors were:",
                    errors.len()
                );

                for error in errors {
                    println!("Failed after {}ms", error.as_millis());
                }
            }
        }
    });
}

async fn timeout<F: Future>(
    max_time: Duration,
    future: F,
) -> Result<F::Output, Duration> {
    match trpl::race(future, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

async fn retry<O, F, T, E>(max_times: u8, op: O) -> Result<T, Vec<E>>
where
    O: Fn() -> F,
    F: Future<Output = Result<T, E>>,
{
    let duration = Duration::from_millis(250);
    let mut errors = vec![];
    for retry in 1..=max_times {
        match op().await {
            Ok(value) => return Ok(value),
            Err(error) => {
                errors.push(error);
                println!("Retrying for the {retry}st/th time");
                trpl::sleep(duration).await;
            }
        }
    }

    Err(errors)
}
