use super::super::component::{TransformComponent, TextureComponent, VelocityComponent};
use super::{System, ReadStorage, WriteStorage};
use std::collections::HashSet;
use std::any::TypeId;
use crate::zip;
use crate::math::Vector2D;

pub struct MovementSystem {
    interests: HashSet<TypeId>
    
}

impl MovementSystem {
    
    pub  fn new() -> Self {
        let vec = vec![TypeId::of::<TransformComponent>(), TypeId::of::<VelocityComponent>()];
        let interests: HashSet<_> = vec.into_iter().collect();
        MovementSystem {interests}
    }
}

impl<'a> System<'a> for MovementSystem {
    type Item = (WriteStorage<'a, TransformComponent>, WriteStorage<'a, VelocityComponent>);
    fn run(&mut self, (poss, vels): Self::Item) {
        
        for (mut pos, vel) in zip!(poss,vels) {
            let old_pos = pos.position();
            pos.set_position(old_pos + Vector2D::new(vel.x() as f32, vel.y() as f32));
        }        
    }

    fn interests(&self) -> &HashSet<TypeId> {
        &self.interests
    }
}