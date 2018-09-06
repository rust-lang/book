
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// tuned example
//fn main() {
//    let s1 = String::from("hello, String");
//
//    let len = calculate_length(&s1);
//
//    println!("The length of '{}' is {}.", s1, len);
//}
//
//fn calculate_length(s: &String) -> u8 {
//    s.len() as u8
//}

// try to modify borrowing string
//fn main() {
//    let s = String::from("hello");
//
//    change(&s);
//}
//
//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}
