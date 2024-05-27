use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        // ANCHOR: move
        let tx_fut = async move {
            // ANCHOR_END: move
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
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                eprintln!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}
