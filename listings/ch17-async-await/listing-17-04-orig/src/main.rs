// ANCHOR: all
fn main() {
    trpl::block_on(async {
        // ANCHOR: add-channel
        let (tx, mut rx) = trpl::channel();
        // ANCHOR_END: add-channel

        // ANCHOR: send-and-receive
        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
        // ANCHOR_END: send-and-receive
    });
}
// ANCHOR_END: all
