// ANCHOR: all
use std::time::Duration;

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

        // ANCHOR: loop
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
        // ANCHOR_END: loop
    });

    println!("Done!");
}
// ANCHOR_END: all
