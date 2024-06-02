#[derive(Clone, Copy)]
pub struct Color(pub u32);

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Screen;

impl Screen {
    pub fn fill_whatever<T>(&mut self, whatever: T) {
        std::hint::black_box(whatever);
    }
}

pub trait Draw {
    fn draw(&self, screen: &mut Screen);
}

#[derive(Clone, Copy)]
pub struct Square {
    pub pos: Point,
    pub w: f32,
    pub color: Color,
}

#[derive(Clone, Copy)]
pub struct Rectangle {
    pub square: Square,
    pub h: f32,
}

trait Area {
    fn area(&self) -> f32;
}

impl Area for Square {
    fn area(&self) -> f32 {
        self.w * self.w
    }
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.h * self.square.w
    }
}

impl Draw for Square {
    fn draw(&self, screen: &mut Screen) {
        screen.fill_whatever(self)
    }
}

impl Draw for Rectangle {
    fn draw(&self, screen: &mut Screen) {
        screen.fill_whatever(self)
    }
}

#[derive(Clone, Copy)]
enum RecOrSquare {
    Rectangle(Rectangle),
    Square(Square),
}

impl Area for RecOrSquare {
    fn area(&self) -> f32 {
        match *self {
            RecOrSquare::Rectangle(rec) => rec.area(),
            RecOrSquare::Square(sq) => sq.area(),
        }
    }
}

impl Draw for RecOrSquare {
    fn draw(&self, screen: &mut Screen) {
        match *self {
            RecOrSquare::Rectangle(rec) => rec.draw(screen),
            RecOrSquare::Square(sq) => sq.draw(screen),
        }
    }
}

#[derive(Clone, Copy)]
enum CircleOrStar {
    Circle(Circle),
    Star(Star),
}

impl Area for RecOrSquare {
    fn area(&self) -> f32 {
        match *self {
            RecOrSquare::Rectangle(rec) => rec.area(),
            RecOrSquare::Square(sq) => sq.area(),
        }
    }
}

impl Draw for RecOrSquare {
    fn draw(&self, screen: &mut Screen) {
        match *self {
            RecOrSquare::Rectangle(rec) => rec.draw(screen),
            RecOrSquare::Square(sq) => sq.draw(screen),
        }
    }
}

