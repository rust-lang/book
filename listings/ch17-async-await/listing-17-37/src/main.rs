fn main() {
    trpl::block_on(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        // ANCHOR: stream
        let mut stream = trpl::stream_from_iter(iter);
        // ANCHOR_END: stream
    });
}
