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

pub struct SpriteComponent {
    texture_sheet: String,
    layer: u32,
    region: sdl2::rect::Rect,
    scale_x: u32,
    scale_y: u32,
    flip_horizontal: bool,
    flip_vertical: bool,
}

impl SpriteComponent {
    pub fn set_texture(&mut self, texture: &str) -> &mut Self{
        self.texture_sheet = texture.to_string();
        self
    }

    pub fn texture(&self) -> &str {
        &self.texture_sheet
    }

    pub fn region(&self) -> sdl2::rect::Rect {
        self.region
    }

    pub fn layer(&self) -> u32 {
        self.layer
    }

    pub fn scale(&self) -> (u32, u32) {
        (self.scale_x, self.scale_y)
    }

    pub fn flip(&self) -> (bool, bool) {
        (self.flip_horizontal, self.flip_vertical)
    }

    pub fn set_region(&mut self, region: sdl2::rect::Rect) -> &mut Self{
        self.region = region;
        self
    }


    pub fn set_scale(&mut self, scale_x: u32, scale_y: u32) -> &mut Self{
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self
    }

    pub fn set_layer(&mut self, layer: u32) -> &mut Self {
        self.layer = layer;
        self
    }

    pub fn set_flip(&mut self, horizontal: bool, vertical: bool) -> &mut Self {
        self.flip_horizontal = horizontal;
        self.flip_vertical = vertical;
        self
    }
}

impl Default for SpriteComponent {
    
    fn default() -> Self {
        SpriteComponent {
            texture_sheet: String::new(), 
            region: sdl2::rect::Rect::new(0, 0, 0, 0),
            scale_x: 1,
            scale_y: 1,
            layer: 0,
            flip_horizontal: false,
            flip_vertical: false,
        }
    }
}

pub struct AnimationData {
    frames: u32,
    speed: u32,
    index: u32,
    flip_horizontal: bool,
    flip_vertical: bool,
}

impl AnimationData {
    pub fn frames(&self) -> u32 {
        self.frames
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn flip(&self) -> (bool, bool) {
        (self.flip_horizontal, self.flip_vertical)
    }

    pub fn set_frames(&mut self, frames: u32) -> &mut Self {
        self.frames = frames;
        self
    }

    pub fn set_speed(&mut self, speed: u32) -> &mut Self {
        self.speed = speed;
        self
    }

    pub fn set_index(&mut self, index: u32) -> &mut Self {
        self.index = index;
        self
    }

    pub fn set_flip(&mut self, horizontal: bool, vertical: bool) -> &mut Self {
        self.flip_horizontal = horizontal;
        self.flip_vertical = vertical;
        self
    }
}


pub struct AnimationComponent {
    animations: std::collections::HashMap<String, AnimationData>,
    active_animation: String
}

impl Default for AnimationComponent {
    fn default() -> Self {
        AnimationComponent {
            animations: std::collections::HashMap::new(),
            active_animation: String::new(),
        }
    }
}

impl AnimationComponent {
    
    pub fn play(&mut self, animation: &str, flip_horizontal: bool, flip_vertical: bool) {
        self.active_animation = animation.to_string();
        self.get_animation_mut(animation).set_flip(flip_horizontal, flip_vertical);
    }

    pub fn animation(&self) -> &AnimationData {
        self.animations.get(&self.active_animation).unwrap()
    }

    pub fn add_animation(&mut self, animation: &str, index: u32, frames: u32, speed: u32) -> &mut Self {
        self.animations.insert(animation.to_string(), AnimationData {
            index,
            frames,
            speed,
            flip_horizontal: false,
            flip_vertical: false,
        });
        self
    }

    pub fn get_animation_mut(&mut self, animation: &str) -> &mut AnimationData {
        self.animations.get_mut(animation).unwrap()
    }
}

pub type KeyboardComponent = Option<sdl2::event::Event>;

pub struct KeyboardControlled;

impl Default for KeyboardControlled {
    
    fn default() -> Self {
        KeyboardControlled
    }
}

pub type Ticks = u64;

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
