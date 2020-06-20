use super::super::component::*;
use super::{System, ReadStorage, WriteStorage};
use crate::zip;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type Item = (WriteStorage<'a, PositionComponent>, ReadStorage<'a, VelocityComponent>);
    fn run(&mut self, (positions, velocities): Self::Item) {
        
        for (mut pos, vel) in zip!(positions, velocities) {
            let old_pos = pos.position();
            pos.set_position(old_pos + vel.velocity());
        } 
    }
}

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type Item = (ReadStorage<'a, PositionComponent>, ReadStorage<'a, CollisionComponent>);
    fn run(&mut self, (positions, sprites): Self::Item) {
        
        let mut bounding_boxes  = vec![];
        let mut tags = vec![];

        for (pos, sprite) in zip!(positions, sprites) {
            tags.push(sprite.tag().to_string());
            bounding_boxes.push(sdl2::rect::Rect::new(pos.position().x() as i32, pos.position().y() as i32, sprite.width(), sprite.height()));
        }  

        for (i, sprite_a) in (&tags).iter().enumerate() {
            for  (j, sprite_b) in tags[i+1..].iter().enumerate() {
                let box_a = &bounding_boxes[i];
                let box_b = &bounding_boxes[i+1+j];
        
                if CollisionDetector::aabb(box_a, box_b) {
                    println!("Collision between {:?} and {:?}", sprite_a, sprite_b);
                }
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