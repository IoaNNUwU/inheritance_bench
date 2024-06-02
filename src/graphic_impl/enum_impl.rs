use super::*;

#[derive(Default)]
pub struct EnumGraphics {
    list: Vec<EnumCompWrapper>
}

impl Draw for EnumGraphics {
    fn draw(&self, screen: &mut Screen) {
        for enum_comp in &self.list {
            enum_comp.draw(screen);
        }
    }
}

impl EnumGraphics {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn add_star(&mut self, star: Star) {
        self.list.push(EnumCompWrapper::from(star));
    }
    pub fn add_rect(&mut self, rect: Rectangle) {
        self.list.push(EnumCompWrapper::from(rect));
    }
    pub fn add_triangle(&mut self, tr: Triangle) {
        self.list.push(EnumCompWrapper::from(tr));
    }
}

enum EnumCompWrapper {
    Rect(Rectangle),
    Trian(Triangle),
    Star(Star),
}

impl Draw for EnumCompWrapper {
    fn draw(&self, screen: &mut Screen) {
        match self {
            EnumCompWrapper::Rect(rect) => rect.draw(screen),
            EnumCompWrapper::Trian(tri) => tri.draw(screen),
            EnumCompWrapper::Star(star) => star.draw(screen),
        }
    }
}

impl From<Rectangle> for EnumCompWrapper {
    fn from(value: Rectangle) -> Self {
        EnumCompWrapper::Rect(value)
    }
}
impl From<Star> for EnumCompWrapper {
    fn from(value: Star) -> Self {
        EnumCompWrapper::Star(value)
    }
}
impl From<Triangle> for EnumCompWrapper {
    fn from(value: Triangle) -> Self {
        EnumCompWrapper::Trian(value)
    }
}