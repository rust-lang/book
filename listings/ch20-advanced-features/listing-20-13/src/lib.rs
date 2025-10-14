pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
