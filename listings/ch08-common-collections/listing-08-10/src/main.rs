fn main() {
    // ANCHOR: here
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
      // ANCHOR_END: here
}
