fn main() {
    // ANCHOR: here
    let mut num = 5;

    let r2 = &mut num as *mut i32;
    let r1 = unsafe { &*r2 as *const i32 };

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    // ANCHOR_END: here
}
