use super::*;

#[derive(Default)]
pub struct DynGraphics {
    list: Vec<Box<dyn Draw>>,
}

impl Draw for DynGraphics {
    fn draw(&self, screen: &mut Screen) {
        for dyn_comp in &self.list {
            dyn_comp.draw(screen);
        }
    }
}

impl DynGraphics {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add<T: Draw + 'static>(&mut self, comp: T) {
        self.list.push(Box::new(comp));
    }
}
