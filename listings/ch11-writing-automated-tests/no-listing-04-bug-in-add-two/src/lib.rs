// ANCHOR: here
pub fn add_two(a: i32) -> i32 {
    a + 3
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }
}
