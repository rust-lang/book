fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

	v.push(6);
    let first = &v[0];

    
    println!("{:?}", v);
	println!("{}", first);
}
