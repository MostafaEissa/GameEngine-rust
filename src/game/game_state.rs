use crate::ecs::{EntityManager, PositionComponent};
use super::graphics::Graphics;
use super::map::Map;
use super::game_object::GameObject;

pub struct GameState {
    is_running: bool,
    
    // temp objects
    player: GameObject,
    enemy: GameObject,
    map: Map,

    pub manager: EntityManager
}

impl GameState {
    
    pub fn new() -> Self {
        GameState {
            is_running: true,
            player: GameObject::new("assets/player.png", 0, 0),
            enemy: GameObject::new("assets/enemy.png", 50, 50),
            map: Map::load_map(),
            manager: EntityManager::new(),
        }
    }

	pub fn render(&self, graphics: &mut Graphics) {

        // clear screen
        graphics.renderer.clear();
        
        self.map.render(graphics);
        self.player.render(graphics);
        self.enemy.render(graphics);
        

		// render on screen
        graphics.renderer.present();
	}

	pub fn update(&mut self) {
        self.player.update();
        self.enemy.update();
        self.manager.update();
        let p = self.manager.get_entity(0).get_component::<PositionComponent>();
        println!("{}, {}", p.x(), p.y());
    }

	pub fn handle_events(&mut self, event: sdl2::event::Event) {	
        match event {
            sdl2::event::Event::Quit{..} => self.set_running(false),
            _ => ()
        }
	}

	pub fn running(&self) -> bool {
		self.is_running
	}

	pub fn set_running(&mut self, running: bool) {
		self.is_running = running;
    }
}