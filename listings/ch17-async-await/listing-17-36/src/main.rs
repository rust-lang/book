fn main() {
    trpl::block_on(async {
        // ANCHOR: range
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        // ANCHOR_END: range
    });
}
