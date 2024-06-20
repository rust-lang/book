// ANCHOR: timeout
use std::time::Duration;
// --snip--
// ANCHOR_END: timeout
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::block_on(async {
        // ANCHOR: timeout

        let mut messages = get_messages().timeout(Duration::from_millis(200));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
        // ANCHOR_END: timeout
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
