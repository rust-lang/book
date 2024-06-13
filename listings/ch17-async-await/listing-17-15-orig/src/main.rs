fn main() {
    trpl::block_on(async {
        // ANCHOR: here
        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };
        // ANCHOR_END: here

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
    });
}
