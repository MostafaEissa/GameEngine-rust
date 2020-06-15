use sdl2::Sdl;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use std::collections::HashMap;

pub struct Game<'a> {
    graphics: Graphics<'a>,
    game_state: GameState,
}

struct Graphics<'a> {
    sdl_context: &'a Sdl,
    renderer: &'a mut Canvas<Window>,
    texture_manager: TextureManager<'a>,
}

impl<'a> Game<'a> {
    pub fn new(sdl_context: &'a Sdl, renderer: &'a mut Canvas<Window>, texture_creator:  &'a TextureCreator<WindowContext>) -> Self {
        
        let graphics = Graphics {
            sdl_context: sdl_context,
            renderer: renderer,
            texture_manager: TextureManager::new(texture_creator),
        };

        let game_state = GameState {
            is_running: true, 
            cnt: 0,
            player: GameObject::new("assets/player.png", 0, 0),
            enemy: GameObject::new("assets/enemy.png", 50, 50),
            map: Map::load_map(),
        };

        Game {
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

struct GameState {
	is_running: bool,
    cnt: i32,
    player: GameObject,
    enemy: GameObject,
    map: Map
}

impl GameState {
    
	fn render(&self, graphics: &mut Graphics) {

        // clear screen
        graphics.renderer.clear();
        
        self.map.render(graphics);
        self.player.render(graphics);
        self.enemy.render(graphics);
        

		// render on screen
        graphics.renderer.present();
	}

	fn update(&mut self) {
        println!("{}", self.cnt);
        self.cnt += 1;
        self.player.update();
        self.enemy.update();
    }

	fn handle_events(&mut self, event: sdl2::event::Event) {	
        match event {
            sdl2::event::Event::Quit{..} => self.set_running(false),
            _ => ()
        }
	}

	fn running(&self) -> bool {
		self.is_running
	}

	fn set_running(&mut self, running: bool) {
		self.is_running = running;
    }
}

struct GameObject {
    xpos: i32,
    ypos: i32,
    src_rect: Rect,
    dest_rect: Rect,
    texture: String
}

impl GameObject {

    fn new(texture_sheet: &str, x: i32, y: i32) -> Self {
        GameObject {
            xpos: x, 
            ypos: y, 
            src_rect: Rect::new(0, 0, 32, 32),
            dest_rect: Rect::new(x, y, 64, 64),
            texture: texture_sheet.to_string()
        }
    }

    fn render(&self, graphics: &mut Graphics) {
        // draw player
        let texture = graphics.texture_manager.load(&self.texture);
        graphics.renderer.copy(&texture, self.src_rect, self.dest_rect).unwrap();
    }

    fn update(&mut self) {
        self.xpos += 1;
        self.ypos += 1;

        self.dest_rect.set_x(self.xpos);
        self.dest_rect.set_y(self.ypos);
    }
}

pub struct TextureManager<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    textures: HashMap<String, Texture<'a>>
}

impl<'a> TextureManager<'a> {

    pub fn new(texture_creator:  &'a TextureCreator<WindowContext>) -> Self {
        TextureManager{texture_creator: texture_creator, textures: HashMap::new()}
    }

    pub fn load(&mut self, path: &str) -> &Texture{
        if self.textures.contains_key(path) {
            return self.textures.get(path).unwrap();
        }

        let texture = self.texture_creator.load_texture(path).unwrap();
        self.textures.insert(path.to_string(), texture);
        self.textures.get(path).unwrap()
    }
}

enum Tile {
    Dirt,
    Grass,
    Water
}

struct Map {
    map: [[Tile; 25]; 20],
}

impl Map {

    fn load_map() -> Self {
        use Tile::*;
        let lvl1 = [
            [
            Water, Water, Water, Grass, Grass, Grass, Grass, Grass, Grass, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Grass, Grass, Grass, Dirt, Dirt, Dirt, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Grass, Grass, Grass, Dirt, Dirt, Dirt, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Grass, Grass, Dirt, Dirt, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Grass, Grass, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
        ];

        Map {map: lvl1}
    }

    fn render(&self, graphics: &mut Graphics) {
        let src = Rect::new(0, 0, 32, 32);
        let mut dest = Rect::new(0, 0, 32, 32);

        for row in 0..20 {
            for column in 0..25 {
                dest.set_x((column as u32 * dest.width()) as i32);
                dest.set_y((row as u32 * dest.height()) as i32);

                match self.map[row][column] {
                    Tile::Water => Self::draw_tile(graphics, "assets/water.png", src, dest),
                    Tile::Dirt => Self::draw_tile(graphics, "assets/dirt.png", src, dest),
                    Tile::Grass => Self::draw_tile(graphics, "assets/grass.png", src, dest),
                }
            }
        }
    }

    fn draw_tile(graphics: &mut Graphics, path: &str, src: Rect, dest: Rect) {
        let texture  = graphics.texture_manager.load(path);
        graphics.renderer.copy(texture, src, dest).unwrap();
    }
}