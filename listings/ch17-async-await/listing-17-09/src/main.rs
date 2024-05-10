fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        // ANCHOR: send-and-receive
        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
        // ANCHOR_END: send-and-receive
    });
}
