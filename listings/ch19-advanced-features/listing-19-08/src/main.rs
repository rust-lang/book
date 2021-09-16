extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // safety: function abs is defined for all inputs
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
