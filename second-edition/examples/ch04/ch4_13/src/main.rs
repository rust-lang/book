fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    let iter = bytes.iter();
   
   
    for (i,&item) in iter.enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let end_index = first_word(&s); // word will get the value 5.
    println!("{}",s.get(0..end_index).unwrap());
    s.clear(); // This empties the String, making it equal to "".

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
