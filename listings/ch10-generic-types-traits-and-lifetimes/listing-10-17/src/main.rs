fn main() {
    // ANCHOR: here
    {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
    // ANCHOR_END: here
}
