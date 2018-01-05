fn main() {
    println!("std::f64");
    // Approximate number of significant digits in base 10.
    println!("DIGITS = {}", std::f64::DIGITS);

    // Difference between 1.0 and the next largest representable number.
    println!("EPSILON = {}", std::f64::EPSILON);

    // Infinity (∞).
    println!("INFINITY = {}", std::f64::INFINITY);

    // Number of significant digits in base 2.
    println!("MANTISSA_DIGITS = {}", std::f64::MANTISSA_DIGITS);

    // Largest finite f64 value.
    println!("MAX = {}", std::f64::MAX);

    // Maximum possible power of 10 exponent.
    println!("MAX_10_EXP = {}", std::f64::MAX_10_EXP);

    // Maximum possible power of 2 exponent.
    println!("MAX_EXP = {}", std::f64::MAX_EXP);

    // Smallest finite f64 value.
    println!("MIN = {}", std::f64::MIN);

    // Minimum possible normal power of 10 exponent.
    println!("MIN_10_EXP = {}", std::f64::MIN_10_EXP);

    // One greater than the minimum possible normal power of 2 exponent.
    println!("MIN_EXP = {}", std::f64::MIN_EXP);

    // Smallest positive normal f64 value.
    println!("MIN_POSITIVE = {}", std::f64::MIN_POSITIVE);

    // Not a Number (NaN).
    println!("NAN = {}", std::f64::NAN);

    // Negative infinity (-∞).
    println!("NEG_INFINITY = {}", std::f64::NEG_INFINITY);

    // The radix or base of the internal representation of f64.
    println!("RADIX = {}", std::f64::RADIX);
}
