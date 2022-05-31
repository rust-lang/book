fn main() {
    // ANCHOR: here
    {
        let x = 5;            // ----------------------+-- 'full_scope
                              //                       |
        let r = &x;           // --+-- 'partial_scope  |
                              //   |                   |
        println!("r: {}", r); //   |                   |
                              // --+                   |
    }                         // ----------------------+
    // ANCHOR_END: here
}
