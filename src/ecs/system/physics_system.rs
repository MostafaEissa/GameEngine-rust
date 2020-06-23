use super::super::component::*;
use super::{System, ReadStorage, WriteStorage, WriteResource};

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type Item = (WriteResource<'a, Camera>, WriteStorage<'a, PositionComponent>, WriteStorage<'a, VelocityComponent>, ReadStorage<'a, CollisionComponent>);

    fn run(&mut self, (mut camera, positions, velocities, sprites): Self::Item) {
        
        let bounding_boxes: Vec<_>= sprites.into_iter().collect();
        let mut poss: Vec<_> = positions.into_iter().collect();
        let mut vels: Vec<_> = velocities.into_iter().collect();

        //update position
        let mut player_pos = poss[0].position();

        for ((pos, vel), sprt) in poss.iter_mut().zip(vels.iter_mut()).zip(bounding_boxes.iter()) {
            let old_pos = pos.position();
            pos.set_position(old_pos + vel.velocity());

            if sprt.tag() == "player" {
                player_pos = old_pos;
            }

            camera.x = pos.position().x() as i32 - 400;
            camera.y = pos.position().y() as i32 - 320;
            if camera.x < 0 {
                camera.x = 0;
            }
            if camera.y < 0 {
                camera.y = 0;
            }
            if camera.x > camera.w as i32{
                camera.x = camera.w as i32;
            }
            if camera.y > camera.h as i32 {
                camera.y = camera.h as i32;
            }
        
        }   
        let mut move_player_back = false;

        // detect collitions
        for (i, ((pos_a, sprite_a), _vel_a)) in poss.iter().zip(bounding_boxes.iter()).zip(vels.iter_mut()).enumerate() {
            for  (j, (pos_b, sprite_b)) in poss.iter().zip(bounding_boxes.iter()).enumerate() {
                
                if i == j || sprite_a.tag() != "player" {continue;}

                let box_a = sdl2::rect::Rect::new(pos_a.position().x() as i32, pos_a.position().y() as i32, sprite_a.width(), sprite_a.height());
                let box_b = sdl2::rect::Rect::new(pos_b.position().x() as i32, pos_b.position().y() as i32, sprite_b.width(), sprite_b.height());
        
                if CollisionDetector::aabb(&box_a, &box_b) {
                    move_player_back = true;
                }
            }
        }
    
        if move_player_back {
            let player =  poss.iter_mut().zip(bounding_boxes.iter()).filter(|item| item.1.tag() == "player").nth(0).unwrap();
            player.0.set_position(player_pos);
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