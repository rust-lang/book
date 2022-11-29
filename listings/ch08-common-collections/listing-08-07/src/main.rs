fn main() {
    // ANCHOR: here
    let v = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;
        println!("{}", n_plus_one);
    }
    // ANCHOR_END: here
}
