use std::any::Any;
pub trait Component : Any {}


pub struct PositionComponent {
    xpos: i32,
    ypos: i32
}

impl PositionComponent {
    
    pub fn x(&self) -> i32 {
        self.xpos
    }

    pub fn y(&self) -> i32 {
        self.ypos
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.xpos = x;
        self.ypos = y;
    }
}

impl Component for PositionComponent {}

impl Default for PositionComponent {
    fn default() -> Self {
        PositionComponent {xpos: 0, ypos: 0}
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

impl Component for VelocityComponent {}

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

impl Component for TextureComponent {}

impl Default for TextureComponent {
    
    fn default() -> Self {
        TextureComponent {
            texture_sheet: String::new(), 
            src_rect: Default::default(),
            dest_rect: Default::default(),
        }
    }
}