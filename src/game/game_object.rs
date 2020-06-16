use super::graphics::Graphics;
use sdl2::rect::Rect; 

pub struct GameObject {
    xpos: i32,
    ypos: i32,
    src_rect: Rect,
    dest_rect: Rect,
    texture: String
}

impl GameObject {

    pub fn new(texture_sheet: &str, x: i32, y: i32) -> Self {
        GameObject {
            xpos: x, 
            ypos: y, 
            src_rect: Rect::new(0, 0, 32, 32),
            dest_rect: Rect::new(x, y, 64, 64),
            texture: texture_sheet.to_string()
        }
    }

    pub fn render(&self, graphics: &mut Graphics) {
        // draw player
        let texture = graphics.texture_manager.load(&self.texture);
        graphics.renderer.copy(&texture, self.src_rect, self.dest_rect).unwrap();
    }

    pub fn update(&mut self) {
        self.xpos += 1;
        self.ypos += 1;

        self.dest_rect.set_x(self.xpos);
        self.dest_rect.set_y(self.ypos);
    }
}
