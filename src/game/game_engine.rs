use sdl2::EventPump;
use crate::ecs::{World, Runnable};


pub struct GameEngine<'a> {
    systems: Vec<Box<dyn Runnable<'a>>>,
}

impl<'a> GameEngine<'a> {

    pub fn new() -> Self {
        GameEngine { systems: vec![]}
    }

    pub fn register<T:'static + Runnable<'a>>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn run(&mut self, world: &'a mut World, mut event_pump: EventPump) {
        'running: 
        loop {
            if let Some(event) = event_pump.poll_event() {
                match event {
                    sdl2::event::Event::Quit{..} => break 'running,
                    _ => ()
                }
            }

            let fps = 60;
            let start = Instant::now();

            for  system in self.systems.iter_mut() {
                system.run(world);
            }

            // adjust frame rate
            use std::time::{Instant, Duration};
            let frame_delay = 1000/ fps ;
            let frame_time = start.elapsed().as_millis() as u64;
            if frame_time < frame_delay {
                std::thread::sleep(Duration::from_millis(frame_delay - frame_time));
            }

        }
    }
}
