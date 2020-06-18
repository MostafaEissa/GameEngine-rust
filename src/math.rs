use std::ops::{Add, Sub, AddAssign, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vector2D {
    x: f32,
    y: f32
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2D {x, y}
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}

impl Add for Vector2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vector2D {x: self.x + other.x, y: self.y + other.y}
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vector2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vector2D {x: self.x - other.x, y: self.y - other.y}
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
