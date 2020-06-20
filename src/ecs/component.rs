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

impl Rect {
    pub fn new(width: u32, height: u32) -> Self {
        Rect {
            width,
            height
        }
    }
}

pub struct SpriteComponent {
    texture_sheet: String,
    region: Rect,
    scale_x: u32,
    scale_y: u32,
}

impl SpriteComponent {
    pub fn set_texture(&mut self, texture: &str) -> &mut Self{
        self.texture_sheet = texture.to_string();
        self
    }

    pub fn texture(&self) -> &str {
        &self.texture_sheet
    }

    pub fn region(&self) -> Rect {
        self.region
    }

    pub fn set_region(&mut self, region: Rect) -> &mut Self{
        self.region = region;
        self
    }

    pub fn scale(&self) -> (u32, u32) {
        (self.scale_x, self.scale_y)
    }

    pub fn set_scale(&mut self, scale_x: u32, scale_y: u32) -> &mut Self{
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self
    }
}

impl Default for SpriteComponent {
    
    fn default() -> Self {
        SpriteComponent {
            texture_sheet: String::new(), 
            region: Default::default(),
            scale_x: 1,
            scale_y: 1
        }
    }
}

pub type KeyboardComponent = Option<sdl2::event::Event>;

pub struct KeyboardControlled;

impl Default for KeyboardControlled {
    
    fn default() -> Self {
        KeyboardControlled
    }
}


pub struct CollisionComponent {
    width: u32,
    height: u32,
    tag: String
}

impl CollisionComponent {
    pub fn width(&self) -> u32 {
        self.width
    } 

    pub fn height(&self) -> u32 {
        self.height
    } 

    pub fn tag(&self) -> &str {
        &self.tag
    } 

    pub fn set_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    } 

    pub fn set_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    } 

    pub fn set_tag(&mut self, tag: &str) -> &mut Self {
        self.tag = tag.to_string();
        self
    }

}

impl Default for CollisionComponent {
    
    fn default() -> Self {
        CollisionComponent {
            width: 0,
            height: 0,
            tag: String::new()
        }
    }
}
