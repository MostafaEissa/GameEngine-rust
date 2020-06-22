use super::super::component::*;
use super::{System, ReadStorage, WriteStorage};

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type Item = (WriteStorage<'a, PositionComponent>, WriteStorage<'a, VelocityComponent>, 
    ReadStorage<'a, CollisionComponent>, ReadStorage<'a, SpriteComponent>);

    fn run(&mut self, (positions, velocities, sprites, texx): Self::Item) {
        
        let bounding_boxes: Vec<_>= sprites.into_iter().collect();
        let mut poss: Vec<_> = positions.into_iter().collect();
        let mut vels: Vec<_> = velocities.into_iter().collect();
        let texs: Vec<_> = texx.into_iter().collect();

    
        // detect collitions
        for (i, ((pos_a, sprite_a), _vel_a)) in poss.iter().zip(bounding_boxes.iter()).zip(vels.iter_mut()).enumerate() {
            for  (j, (pos_b, sprite_b)) in poss.iter().zip(bounding_boxes.iter()).enumerate() {
                
                if i == j {continue;}

                let box_a = sdl2::rect::Rect::new(pos_a.position().x() as i32, pos_a.position().y() as i32, sprite_a.width(), sprite_a.height());
                let box_b = sdl2::rect::Rect::new(pos_b.position().x() as i32, pos_b.position().y() as i32, sprite_b.width(), sprite_b.height());
        
                if CollisionDetector::aabb(&box_a, &box_b) {
                    println!("collision between {} and {}", sprite_a.tag(), sprite_b.tag());
                }
            }
        }
    
        //update position
        let player = poss.iter_mut().zip(vels.iter_mut()).zip(texs.iter()).filter(
            |item| item.1.layer() == 1
        ).nth(0).unwrap();
        let velocity = (player.0).1.velocity();

        for ((pos, _), tex) in poss.iter_mut().zip(vels.iter_mut()).zip(texs.iter()) {
            if tex.layer() == 0 {
                let old_pos = pos.position();
                pos.set_position(old_pos - velocity);
            }
        }
        
    }
}

struct CollisionDetector;

impl CollisionDetector {
    fn aabb(rec_a: &sdl2::rect::Rect, rec_b: &sdl2::rect::Rect) -> bool {
        if rec_a.x() + rec_a.width() as i32 >= rec_b.x() &&
        rec_b.x() + rec_b.width() as i32 >= rec_a.x() &&
        rec_a.y() + rec_a.height() as i32 >= rec_b.y() &&
        rec_b.y() + rec_b.height() as i32 >= rec_a.y() {
            return true;
        }
        return false;
    }
}