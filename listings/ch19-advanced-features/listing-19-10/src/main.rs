static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // safety: only 1 thread can increment COUNTER (assumes no _start hackery)
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    // safety: only 1 thread accesses COUNTER (assumes no _start hackery)
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
