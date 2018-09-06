fn main() {
    let max = <i8>::max_value();
    let mut value = <i8>::min_value();
    loop {
        value = value + 1;

        println!("value = {}", value);

        if value == max {
            break;
        }
    }
}
