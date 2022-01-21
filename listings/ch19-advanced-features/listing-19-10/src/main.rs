static mut COUNTER: u32 = 0;

/// SAFETY: Must not be used to trigger data races
unsafe fn add_to_count(inc: u32) {
    COUNTER += inc;
}

fn main() {
    unsafe {
        add_to_count(3);
        println!("COUNTER: {}", COUNTER);
    }
}
