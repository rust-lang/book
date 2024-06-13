// ANCHOR: all
// ANCHOR: many-messages
use std::time::Duration;
// ANCHOR_END: many-messages

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        // ANCHOR: many-messages
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_secs(1)).await;
        }
        // ANCHOR_END: many-messages

        // ANCHOR: loop
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
        // ANCHOR_END: loop
    });
}
// ANCHOR_END: all
