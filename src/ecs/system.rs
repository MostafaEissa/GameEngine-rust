use crate::game::Graphics;
use sdl2::rect::Rect;
use std::collections::HashSet;
use std::any::TypeId;
use super::entity::{EntityManager};
use super::component::{PositionComponent, TextureComponent, VelocityComponent};

pub struct RenderSystem {
    interests: HashSet<TypeId>
}

impl RenderSystem {
    pub fn render(&self, graphics: &mut Graphics, manager: &EntityManager) {
        for entity in manager.filter(&self.interests) {
            let position = manager.get_component::<PositionComponent>(entity);
            let texture_sheet = manager.get_component::<TextureComponent>(entity);
            let texture = graphics.texture_manager.load(texture_sheet.texture());
            let src_rect = texture_sheet.src_rect();
            let dest_rect = texture_sheet.dest_rect();

            let src = Rect::new(0, 0, src_rect.width, src_rect.height);
            let dest = Rect::new(position.x(), position.y(), dest_rect.width, dest_rect.height);
            graphics.renderer.copy(texture, src, dest).unwrap();
        }
    }
}

impl Default for RenderSystem {
    fn default() -> Self {
        let vec = vec![TypeId::of::<PositionComponent>(), TypeId::of::<TextureComponent>()];
        let interests: HashSet<_> = vec.into_iter().collect();
        RenderSystem {interests}
    }
}

pub struct MovementSystem {
    interests: HashSet<TypeId>
}

impl MovementSystem {
    pub fn update(&self, manager: &mut EntityManager) {
        for entity in manager.filter(&self.interests) {
            let velocity = manager.get_component::<VelocityComponent>(entity);
            let vx = velocity.x();
            let vy = velocity.y();

            let position = manager.get_component_mut::<PositionComponent>(entity);
            position.set_pos(position.x() + vx, position.y() + vy);
            let new_xpos = position.x();

            let texture_sheet = manager.get_component_mut::<TextureComponent>(entity);
            if new_xpos > 100 {
                texture_sheet.set_texture("assets/enemy.png", texture_sheet.src_rect(), texture_sheet.dest_rect());
            }

        }
    }
}

impl Default for MovementSystem {
    fn default() -> Self {
        let vec = vec![TypeId::of::<PositionComponent>(), TypeId::of::<VelocityComponent>(), TypeId::of::<TextureComponent>()];
        let interests: HashSet<_> = vec.into_iter().collect();
        MovementSystem {interests}
    }
}