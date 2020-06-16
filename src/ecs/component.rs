use std::any::Any;


pub trait Component : Any {
    //fn new() -> Self;
    fn update(&mut self) {}
    fn draw(&self) {}
    fn as_any(&mut self) -> &mut dyn Any;
}

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

impl Component for PositionComponent {
    fn update(&mut self) {
        self.xpos += 1;
        self.ypos += 1;
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for PositionComponent {
    fn default() -> Self {
        PositionComponent {xpos: 0, ypos: 0}
    }
}