use sdl2::Sdl;
use sdl2::image::LoadSurface;
use sdl2::render::{Canvas, Texture};
use sdl2::video::{Window};

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

    pub fn start(&mut self) {

        //create player textures
        let texture_creator = self.renderer.texture_creator();
        let surface = sdl2::surface::Surface::from_file("assets/player.png").unwrap();
        let player_texture = surface.as_texture(&texture_creator).unwrap();

        while self.running() {
            self.handle_events();
            self.update();
            self.render(&player_texture);
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