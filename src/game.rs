pub struct Game {
	is_running: bool,
	renderer: sdl2::render::Canvas<sdl2::video::Window>,
	sdl_context: sdl2::Sdl
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

		Ok(Game {is_running: true, sdl_context: sdl, renderer: renderer})
	}

	pub fn render(&mut self) {
		// clear screen
		self.renderer.clear();

		// render on screen
		self.renderer.present();
	
	}

	pub fn update(&mut self) {

    }

	pub fn handle_events(&mut self) {
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