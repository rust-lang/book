fn main() {
    println!("std::f32");
    // Approximate number of significant digits in base 10.
    println!("DIGITS = {}", std::f32::DIGITS);

    // Difference between 1.0 and the next largest representable number.
    println!("EPSILON = {}", std::f32::EPSILON);

    // Infinity (∞).
    println!("INFINITY = {}", std::f32::INFINITY);

    // Number of significant digits in base 2.
    println!("MANTISSA_DIGITS = {}", std::f32::MANTISSA_DIGITS);

    // Largest finite f32 value.
    println!("MAX = {}", std::f32::MAX);

    // Maximum possible power of 10 exponent.
    println!("MAX_10_EXP = {}", std::f32::MAX_10_EXP);

    // Maximum possible power of 2 exponent.
    println!("MAX_EXP = {}", std::f32::MAX_EXP);

    // Smallest finite f32 value.
    println!("MIN = {}", std::f32::MIN);

    // Minimum possible normal power of 10 exponent.
    println!("MIN_10_EXP = {}", std::f32::MIN_10_EXP);

    // One greater than the minimum possible normal power of 2 exponent.
    println!("MIN_EXP = {}", std::f32::MIN_EXP);

    // Smallest positive normal f32 value.
    println!("MIN_POSITIVE = {}", std::f32::MIN_POSITIVE);

    // Not a Number (NaN).
    println!("NAN = {}", std::f32::NAN);

    // Negative infinity (-∞).
    println!("NEG_INFINITY = {}", std::f32::NEG_INFINITY);

    // The radix or base of the internal representation of f32.
    println!("RADIX = {}", std::f32::RADIX);
}
