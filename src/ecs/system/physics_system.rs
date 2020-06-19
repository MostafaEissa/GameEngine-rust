use super::super::component::*;
use super::{System, ReadStorage, WriteStorage};
use std::collections::HashSet;
use std::any::TypeId;
use crate::zip;

pub struct PhysicsSystem {
    interests: HashSet<TypeId>
    
}

impl PhysicsSystem {
    
    pub  fn new() -> Self {
        let vec = vec![TypeId::of::<PositionComponent>(), TypeId::of::<VelocityComponent>()];
        let interests: HashSet<_> = vec.into_iter().collect();
        PhysicsSystem {interests}
    }
}

impl<'a> System<'a> for PhysicsSystem {
    type Item = (WriteStorage<'a, PositionComponent>, ReadStorage<'a, VelocityComponent>);
    fn run(&mut self, (positions, velocities): Self::Item) {
        
        for (mut pos, vel) in zip!(positions, velocities) {
            let old_pos = pos.position();
            pos.set_position(old_pos + vel.velocity());
        }        
    }

    fn interests(&self) -> &HashSet<TypeId> {
        &self.interests
    }
}