extern crate trpl; // required for mdbook test

// ANCHOR: here
use std::pin::{Pin, pin};

// -- snip --

// ANCHOR_END: here
use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        // ANCHOR: here
        let tx1_fut = pin!(async move {
            // --snip--
            // ANCHOR_END: here
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
            // ANCHOR: here
        });

        // ANCHOR_END: here
        // ANCHOR: here
        let rx_fut = pin!(async {
            // --snip--
            // ANCHOR_END: here
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
            // ANCHOR: here
        });

        let tx_fut = pin!(async move {
            // --snip--
            // ANCHOR_END: here
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
            // ANCHOR: here
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];
        // ANCHOR_END: here

        trpl::join_all(futures).await;
    });
}
