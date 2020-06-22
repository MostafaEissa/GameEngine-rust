use sdl2::Sdl;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;
use crate::game::{Graphics, TextureManager};

use super::super::component::*;
use super::{System, ReadStorage, WriteStorage, ReadResource};
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

            let dest = Rect::new(pos.position().x() as i32 , pos.position().y() as i32, src_rect.w as u32 * scale_x, src_rect.h as u32 * scale_y);

            self.graphics.renderer.copy_ex(texture, src_rect, dest, 0.0, None, tex.flip().0, tex.flip().1).unwrap();
        }
        
        self.graphics.renderer.present();
    }
}

pub struct AnimationSystem;

impl<'a> System<'a> for AnimationSystem {
    type Item = (ReadResource<'a, Ticks>, ReadStorage<'a, AnimationComponent>, WriteStorage<'a, SpriteComponent>);
    fn run(&mut self, (ticks, animations, sprites): Self::Item) {

        for (animator, mut sprite) in zip!(animations, sprites) {
            let mut region = sprite.region();
            let animation = animator.animation();

            region.y = (animation.index() * region.h as u32) as i32;
            region.x = ((region.w as u64) * (((*ticks) / (animation.speed() as u64)) % (animation.frames() as u64))) as i32;
            sprite.set_region(region);
            sprite.set_flip(animation.flip().0, animation.flip().1);
        }
    }
}
