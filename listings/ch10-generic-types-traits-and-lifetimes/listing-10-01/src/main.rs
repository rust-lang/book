// ANCHOR: here
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    // ANCHOR_END: here
    assert_eq!(largest, 100);
    // ANCHOR: here
}
// ANCHOR_END: here
