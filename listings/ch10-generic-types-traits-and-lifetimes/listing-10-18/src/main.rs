fn main() {
    let x = 5;            // ----------+-- 'b ('b is the lifetime of `x`)
                          //           |
    let r = &x;           // --+-- 'a  |      ('a is the lifetime of `r`)
                          //   |       |
    println!("r: {r}");   //   |       |      (use the lifetime of `r`)
                          //   |       |
}                         // --+-------+      (end of both `r` and `x`)
