// ANCHOR: pin
use std::{pin::pin, time::Duration};
// --snip--

// ANCHOR_END: pin
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::block_on(async {
        // ANCHOR: pin
        let mut messages =
            pin!(get_messages().timeout(Duration::from_millis(200)));
        // ANCHOR_END: pin

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
