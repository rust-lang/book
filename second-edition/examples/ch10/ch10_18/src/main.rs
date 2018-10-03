// illigal: `x` does not live long enough
fn main(){
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
