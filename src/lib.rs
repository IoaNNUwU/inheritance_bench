pub mod graphic_impl;
pub use graphic_impl::*;

mod screen;
pub use screen::Screen;

mod component;
pub use component::*;


pub trait Draw {
    fn draw(&self, screen: &mut Screen);
}


#[derive(Debug, Clone, Copy)]
pub struct Color(pub u32);

impl Color {
    pub const RED: Color = Color(0xFF000000);
    pub const GREEN: Color = Color(0x00FF0000);
    pub const BLUE: Color = Color(0x0000FF00);
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}