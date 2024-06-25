use std::{pin::pin, thread, time::Duration};

use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::block_on(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval #{count}"))
            .throttle(Duration::from_millis(500))
            .timeout(Duration::from_secs(10));

        let mut merged = pin!(messages.merge(intervals).take(20));

        while let Some(result) = merged.next().await {
            match result {
                Ok(item) => println!("{item}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

// ANCHOR: thread
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    thread::spawn(move || {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            thread::sleep(Duration::from_millis(time_to_sleep));

            if let Err(send_error) =
                tx.send(format!("Message: '{message}' after {time_to_sleep}ms"))
            {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    thread::spawn(move || {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_millis(1));
            count += 1;
            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
// ANCHOR_END: thread
