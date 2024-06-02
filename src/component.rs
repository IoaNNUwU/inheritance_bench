use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub pos: Point,
    pub w: f32,
    pub color: Color,
}

impl Draw for Square {
    fn draw(&self, screen: &mut Screen) {
        screen.fill_rect(Rectangle {
            pos: self.pos,
            w: self.w,
            h: self.w,
            color: self.color,
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub pos: Point,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

impl Draw for Rectangle {
    fn draw(&self, screen: &mut Screen) {
        screen.fill_rect(*self);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub color: Color,
}

impl Draw for Triangle {
    fn draw(&self, screen: &mut Screen) {
        screen.fill_triangle(*self);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub pos: Point,
    pub radius: f32,
    pub color: Color,
}

impl Draw for Circle {
    fn draw(&self, screen: &mut Screen) {
        screen.fill_whatever(*self);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Star {
    pub pos: Point,
    pub radius: f32,
    pub angle: f32,
    pub color: Color,
}

impl Draw for Star {
    fn draw(&self, screen: &mut Screen) {
        screen.fill_whatever(*self);
    }
}
