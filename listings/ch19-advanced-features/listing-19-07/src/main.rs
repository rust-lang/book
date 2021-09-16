fn main() {
    // ANCHOR: here
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    // pointer does not point to valid object, so access crashes program
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // ANCHOR_END: here
}
