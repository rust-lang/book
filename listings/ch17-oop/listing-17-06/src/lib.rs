pub trait Draw {
    fn draw(&self);
}

// ANCHOR: here
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
// ANCHOR_END: here
