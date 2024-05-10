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

    ReceiverStream::new(rx)
}
