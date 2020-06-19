use std::any::Any;
use crate::math::Vector2D;

pub trait Component : Any {}

impl<T: 'static> Component for T {}


pub struct PositionComponent {
    position: Vector2D,
}

impl PositionComponent {
    
    pub fn position(&self) -> Vector2D {
        self.position
    }

    pub fn set_position(&mut self, position: Vector2D) {
        self.position = position;
    }
}

impl Default for PositionComponent {
    fn default() -> Self {
        PositionComponent {position: Vector2D::new(0.0, 0.0)}
    }
}

pub struct VelocityComponent {
    direction: Vector2D,
    speed: f32
}

impl VelocityComponent {
    
    pub fn direction(&self) -> Vector2D {
        self.direction
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn velocity(&self) -> Vector2D {
        self.direction * self.speed
    }

    pub fn set_direction(&mut self, direction: Vector2D) {
        self.direction = direction;
    }
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn set_velocity(&mut self, speed: f32, direction: Vector2D) {
        self.speed = speed;
        self.direction = direction;
    }
}

impl Default for VelocityComponent {
    fn default() -> Self {
        VelocityComponent {speed: 0.0, direction: Vector2D::new(0.0, 0.0)}
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


pub type KeyboardComponent = Option<sdl2::event::Event>;

