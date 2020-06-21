use sdl2::Sdl;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;
use crate::game::{Graphics, TextureManager};

use super::super::component::*;
use super::{System, ReadStorage};
use crate::zip;

pub struct RenderSystem {
    graphics: Graphics,
}

impl RenderSystem{
    pub fn new(sdl_context: Sdl, renderer: Canvas<Window>, texture_creator:  TextureCreator<WindowContext>) -> Self {
        RenderSystem {
            graphics:   
            Graphics {
                sdl_context: sdl_context,
                renderer: renderer,
                texture_manager: TextureManager::new(texture_creator)
            }
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type Item = (ReadStorage<'a, PositionComponent>, ReadStorage<'a, SpriteComponent>);
    fn run(&mut self, (poss, texs): Self::Item) {
        
        self.graphics.renderer.set_draw_color(sdl2::pixels::Color::WHITE);
        self.graphics.renderer.clear();

        let mut renderables: Vec<_> = zip!(poss, texs).collect();
        renderables.sort_by_key(|item| item.1.layer());

        for (pos, tex) in renderables {
            let texture = self.graphics.texture_manager.load(tex.texture());
            let src_rect = tex.region();
            let (scale_x, scale_y) = tex.scale();

            let src = Rect::new(src_rect.x, src_rect.y, src_rect.width, src_rect.height);
            let dest = Rect::new(pos.position().x() as i32 , pos.position().y() as i32, src_rect.width * scale_x, src_rect.height * scale_y);

            self.graphics.renderer.copy(texture, src, dest).unwrap();
        }
        
        self.graphics.renderer.present();
    }
}