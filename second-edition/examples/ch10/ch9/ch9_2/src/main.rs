use std::fs::File;

fn main() {
	let f = File::open("hello.txt");
	//let f: u32 = File::open("hello.txt");
    println!("{:?}",f);
}
