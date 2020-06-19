use super::super::component::*;
use super::{System, WriteStorage, ReadResource};
use std::collections::HashSet;
use std::any::TypeId;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct KeyboardSystem {
    interests: HashSet<TypeId>
    
}

impl KeyboardSystem {
    
    pub  fn new() -> Self {
        let vec = vec![TypeId::of::<VelocityComponent>()];
        let interests: HashSet<_> = vec.into_iter().collect();
        KeyboardSystem {interests}
    }
}

impl<'a> System<'a> for KeyboardSystem {
    type Item = (ReadResource<'a, KeyboardComponent>, WriteStorage<'a, VelocityComponent>);
    fn run(&mut self, (evt, velocities): Self::Item) {
        if let Some(ref event) = *evt {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Left), .. }  => {
                    for mut vel in velocities {
                        let mut new_direction = vel.direction();
                        new_direction.set_x(-1.0);
                        vel.set_direction(new_direction);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. }  => {
                    for mut vel in velocities {
                        let mut new_direction = vel.direction();
                        new_direction.set_x(1.0);
                        vel.set_direction(new_direction);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. }  => {
                    for mut vel in velocities {
                        let mut new_direction = vel.direction();
                        new_direction.set_y(-1.0);
                        vel.set_direction(new_direction);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. }  => {
                    for mut vel in velocities {
                        let mut new_direction = vel.direction();
                        new_direction.set_y(1.0);
                        vel.set_direction(new_direction);
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    for mut vel in velocities {
                        let mut new_direction = vel.direction();
                        new_direction.set_x(0.0);
                        vel.set_direction(new_direction);
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    for mut vel in velocities {
                        let mut new_direction = vel.direction();
                        new_direction.set_x(0.0);
                        vel.set_direction(new_direction);
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    for mut vel in velocities {
                        let mut new_direction = vel.direction();
                        new_direction.set_y(0.0);
                        vel.set_direction(new_direction);
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Down), .. }  => {
                    for mut vel in velocities {
                        let mut new_direction = vel.direction();
                        new_direction.set_y(0.0);
                        vel.set_direction(new_direction);
                    }
                }
                _ => {}
            }
        }
    }

    fn interests(&self) -> &HashSet<TypeId> {
        &self.interests
    }
}