use super::super::component::*;
use super::{System, WriteStorage, ReadStorage, ReadResource};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::zip;

pub struct KeyboardSystem;

impl<'a> System<'a> for KeyboardSystem {
    type Item = (ReadResource<'a, KeyboardComponent>, ReadStorage<'a, KeyboardControlled>, WriteStorage<'a, VelocityComponent>, WriteStorage<'a, AnimationComponent>);
    fn run(&mut self, (evt, _, velocities, animators): Self::Item) {
        if let Some(ref event) = *evt {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Left), .. }  => {
                    for (mut vel, mut animator) in zip!(velocities, animators) {
                        let mut new_direction = vel.direction();
                        new_direction.set_x(-1.0);
                        vel.set_direction(new_direction);
                        animator.play("Walk", true, false);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. }  => {
                    for(mut vel, mut animator) in zip!(velocities, animators) {
                        let mut new_direction = vel.direction();
                        new_direction.set_x(1.0);
                        vel.set_direction(new_direction);
                        animator.play("Walk", false, false);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. }  => {
                    for (mut vel, mut animator) in zip!(velocities, animators) {
                        let mut new_direction = vel.direction();
                        new_direction.set_y(-1.0);
                        vel.set_direction(new_direction);
                        animator.play("Walk", false, false);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. }  => {
                    for (mut vel, mut animator) in zip!(velocities, animators) {
                        let mut new_direction = vel.direction();
                        new_direction.set_y(1.0);
                        vel.set_direction(new_direction);
                        animator.play("Walk", false, false);
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    for(mut vel, mut animator) in zip!(velocities, animators) {
                        let mut new_direction = vel.direction();
                        new_direction.set_x(0.0);
                        vel.set_direction(new_direction);
                        animator.play("Idle", false, false);
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    for (mut vel, mut animator) in zip!(velocities, animators) {
                        let mut new_direction = vel.direction();
                        new_direction.set_x(0.0);
                        vel.set_direction(new_direction);
                        animator.play("Idle", false, false);
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    for (mut vel, mut animator) in zip!(velocities, animators) {
                        let mut new_direction = vel.direction();
                        new_direction.set_y(0.0);
                        vel.set_direction(new_direction);
                        animator.play("Idle", false, false);
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Down), .. }  => {
                    for(mut vel, mut animator) in zip!(velocities, animators) {
                        let mut new_direction = vel.direction();
                        new_direction.set_y(0.0);
                        vel.set_direction(new_direction);
                        animator.play("Idle", false, false);
                    }
                }
                _ => {}
            }
        }
    }
}