fn main() {
    // ANCHOR: here
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // safety: r1 and r2 point to the same i32 variable on the stack
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    // ANCHOR_END: here
}
