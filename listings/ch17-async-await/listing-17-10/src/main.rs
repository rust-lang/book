use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        // ANCHOR: futures
        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            // ANCHOR: loop
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
            // ANCHOR_END: loop
        };

        trpl::join(tx_fut, rx_fut).await;
        // ANCHOR_END: futures
    });
}
