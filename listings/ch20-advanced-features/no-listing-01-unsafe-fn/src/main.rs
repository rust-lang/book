fn main() {
    // ANCHOR: here
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
    // ANCHOR_END: here
}
