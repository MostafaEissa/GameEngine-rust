use sdl2::Sdl;
use sdl2::render::{Canvas, Texture};
use sdl2::video::{Window};

use super::texture::TextureManager;

pub struct Game {
	is_running: bool,
    renderer: Canvas<Window>,
    sdl_context: Sdl,

    cnt: i32,
    dest_rect: sdl2::rect::Rect
}

impl Game {
	pub fn init(title: &str, width: u32, height: u32, fullscreen: bool) -> Result<Self, String>{
		
		// initialize SDL
		let sdl = sdl2::init()?;
		
		println!("Subsystems initialized!...");
		
		// create window
		let video_subsystem = sdl.video()?;
		let mut window = video_subsystem.window(title, width, height);
		
		//set position
		window.position_centered();

		// apply fullscreen
		if fullscreen {
			window.fullscreen();
		} 

		let window = window.build().map_err(|err| err.to_string())?;
		println!("Window created!...");
		
		// create renderer
        let mut renderer = window.into_canvas().build().map_err(|err| err.to_string())?;
		println!("Renderer created!...");

		// set drawing color
        renderer.set_draw_color(sdl2::pixels::Color::WHITE);

        Ok(Game {
            is_running: true, 
            sdl_context: sdl, 
            renderer: renderer,
            cnt: 0,
            dest_rect: sdl2::rect::Rect::new(0, 0, 64, 64)
        })
    }

    pub fn start(&mut self, fps: u64) {

        //create player textures
        let texture_creator = self.renderer.texture_creator();
        let texture_manager = TextureManager::new(&texture_creator);

        let player_texture = texture_manager.load_texture("assets/player.png");

        while self.running() {

            let start = Instant::now();

            self.handle_events();
            self.update();
            self.render(&player_texture);

            // adjust frame rate
            use std::time::{Instant, Duration};
            let frame_delay = 1000/ fps ;
            let frame_time = start.elapsed().as_millis() as u64;
            if frame_time < frame_delay {
                std::thread::sleep(Duration::from_millis(frame_delay - frame_time));
            }
        }
    }

	fn render(&mut self, player_texture: &Texture) {
		// clear screen
		self.renderer.clear();

        // draw player
        self.renderer.copy(player_texture, None, self.dest_rect).unwrap();

		// render on screen
		self.renderer.present();
	
	}

	fn update(&mut self) {
        println!("{}", self.cnt);
        self.cnt += 1;

        self.dest_rect.set_x(self.cnt);
    }

	fn handle_events(&mut self) {
		let mut event_pump = self.sdl_context.event_pump().unwrap();
		if let Some(event) = event_pump.poll_event() {
			match event {
				sdl2::event::Event::Quit{..} => self.set_running(false),
				_ => ()
			}
		}
	}

	pub fn running(&self) -> bool {
		self.is_running
	}

	fn set_running(&mut self, running: bool) {
		self.is_running = running;
    }
}