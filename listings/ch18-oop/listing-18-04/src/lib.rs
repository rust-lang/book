pub trait Draw {
    fn draw(&self);
}

// ANCHOR: here
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
// ANCHOR_END: here
