use sdl2::Sdl;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use super::graphics::Graphics;
use super::game_state::GameState;
use super::texture_manager::TextureManager;


pub struct GameEngine<'a> {
    graphics: Graphics<'a>,
    game_state: GameState,
}

impl<'a> GameEngine<'a> {
    pub fn new(sdl_context: &'a Sdl, renderer: &'a mut Canvas<Window>, texture_creator:  &'a TextureCreator<WindowContext>) -> Self {
        
        let graphics = Graphics {
            sdl_context: sdl_context,
            renderer: renderer,
            texture_manager: TextureManager::new(texture_creator),
        };

        let game_state = GameState::new();

        GameEngine {
            graphics: graphics,
            game_state: game_state,
        }
    }

    pub fn running(&self) -> bool {
        self.game_state.running()
    }

    pub fn handle_events(&mut self) {
        let mut event_pump = self.graphics.sdl_context.event_pump().unwrap();
        if let Some(event) = event_pump.poll_event() {
            self.game_state.handle_events(event);
		}
    }

    pub fn render(&mut self) {
        let fps = 60;
        let start = Instant::now();
        self.game_state.render(&mut self.graphics);
        
        // adjust frame rate
        use std::time::{Instant, Duration};
        let frame_delay = 1000/ fps ;
        let frame_time = start.elapsed().as_millis() as u64;
        if frame_time < frame_delay {
            std::thread::sleep(Duration::from_millis(frame_delay - frame_time));
        }
    } 

    pub fn update(&mut self) {
        self.game_state.update();
    }
    
}
