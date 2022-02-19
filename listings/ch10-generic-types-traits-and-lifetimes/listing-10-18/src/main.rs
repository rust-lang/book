fn main() {
    // ANCHOR: here
    {
        let r;                // -------------------+-- 'outer_scope
                              //                    |
        {                     //                    |
            let x = 5;        // -+-- 'inner_scope  |
            r = &x;           //  |                 |
        }                     // -+                 |
                              //                    |
        println!("r: {}", r); //                    |
    }                         // -------------------+
    // ANCHOR_END: here
}
