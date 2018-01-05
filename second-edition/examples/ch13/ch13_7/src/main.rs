extern crate ch13_7;
use ch13_7::Shoe;
fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = ch13_7::shoes_in_my_size(shoes, 10);
    println!("{:#?}", in_my_size);
}
