use std::{boxed::Box, future::Future, pin::Pin, time::Duration};

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async {
            println!("Sending 'Hello'");
            tx.send("Hello").unwrap();

            println!("Sleeping!");
            trpl::sleep(Duration::from_millis(1)).await;

            println!("Sending 'Goodbye'");
            tx.send("Goodbye").unwrap();
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // ANCHOR: updated
        let tx_fut2 = async {
            println!("Sending 'Extra'");
            tx.send("Extra").unwrap();

            println!("Sleeping from tx_fut2");
            trpl::sleep(Duration::from_millis(1)).await;
        };

        trpl::join3(tx_fut, tx_fut2, rx_fut).await;
        // ANCHOR_END: updated
    });

    println!("Done!");
}
