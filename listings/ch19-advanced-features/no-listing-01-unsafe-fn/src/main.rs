fn main() {
    // ANCHOR: here
    // assume: This function has to be called from main()
    unsafe fn dangerous() {}

    // safety: function dangerous() is called from main()
    unsafe {
        dangerous();
    }
    // ANCHOR_END: here
}
