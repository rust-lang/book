use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::block_on(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    })
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    // ANCHOR: send
    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }
    // ANCHOR_END: send

    ReceiverStream::new(rx)
}
