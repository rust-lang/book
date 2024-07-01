// ANCHOR: all
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
// ANCHOR_END: all
