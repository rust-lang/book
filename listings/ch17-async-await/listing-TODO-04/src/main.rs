use std::time::Duration;

fn main() {
    trpl::block_on(async {
        async {
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
                    eprintln!("received '{value}'");
                }
            };

            trpl::join(tx_fut, rx_fut).await;
        }
        .await;
    });

    println!("Done!");
}
