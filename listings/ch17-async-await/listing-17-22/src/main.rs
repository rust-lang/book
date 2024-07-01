fn main() {
    trpl::block_on({
        // ANCHOR: here
        async {
            let mut strings = vec![];

            let a = trpl::read_to_string("test-data/hello.txt").await.unwrap();
            strings.push(a.trim());

            let b = trpl::read_to_string("test-data/world.txt").await.unwrap();
            strings.push(b.trim());

            let combined = strings.join(" ");
            println!("{combined}");
        }
        // ANCHOR_END: here
    });
}
