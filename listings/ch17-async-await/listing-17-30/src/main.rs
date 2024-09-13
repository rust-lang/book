extern crate trpl; // required for mdbook test

fn main() {
    trpl::run(async {
        // ANCHOR: stream
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
        // ANCHOR_END: stream
    });
}
