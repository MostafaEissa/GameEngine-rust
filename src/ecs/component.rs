use std::any::Any;
use crate::math::Vector2D;

pub trait Component : Any {}

impl<T: 'static> Component for T {}


pub struct TransformComponent {
    position: Vector2D,
}

impl TransformComponent {
    
    pub fn position(&self) -> Vector2D {
        self.position
    }

    pub fn set_position(&mut self, position: Vector2D) {
        self.position = position;
    }
}

impl Default for TransformComponent {
    fn default() -> Self {
        TransformComponent {position: Vector2D::new(0.0, 0.0)}
    }
}

pub struct VelocityComponent {
    x: i32,
    y: i32,
}

impl VelocityComponent {
    
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn set_velocity(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

impl Default for VelocityComponent {
    fn default() -> Self {
        VelocityComponent {x: 0, y: 0}
    }
}

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub width: u32,
    pub height: u32
}

pub struct TextureComponent {
    texture_sheet: String,
    src_rect: Rect,
    dest_rect: Rect,
}

impl TextureComponent {
    pub fn set_texture(&mut self, texture: &str, src_rect: Rect, dest_rect: Rect) {
        self.texture_sheet = texture.to_string();
        self.src_rect = src_rect;
        self.dest_rect = dest_rect;
    }

    pub fn texture(&self) -> &str {
        &self.texture_sheet
    }

    pub fn src_rect(&self) -> Rect {
        self.src_rect
    }

    pub fn dest_rect(&self) -> Rect {
        self.dest_rect
    }
}

impl Default for TextureComponent {
    
    fn default() -> Self {
        TextureComponent {
            texture_sheet: String::new(), 
            src_rect: Default::default(),
            dest_rect: Default::default(),
        }
    }
}