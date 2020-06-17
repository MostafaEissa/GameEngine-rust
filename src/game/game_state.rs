use crate::ecs::{EntityManager, RenderSystem, MovementSystem, Rect};
use crate::ecs::{TextureComponent, PositionComponent, VelocityComponent};
use super::graphics::Graphics;
use super::map::Map;

pub struct GameState {
    is_running: bool,
    map: Map,
    manager: EntityManager,
    render_system: RenderSystem,
    movement_system: MovementSystem,
}

impl GameState {
    
    pub fn new() -> Self {
        let mut new_game = GameState {
            is_running: true,
            map: Map::load_map(),
            manager: EntityManager::default(),
            render_system: RenderSystem::default(),
            movement_system: MovementSystem::default(),
        };

        let player = new_game.manager.create_entity();
        let texture_component = new_game.manager.add_component::<TextureComponent>(player);
        texture_component.set_texture("assets/player.png", Rect {width: 32, height: 32}, Rect {width: 64, height: 64});
        let position_component = new_game.manager.add_component::<PositionComponent>(player);
        position_component.set_pos(0, 300);
        let velocity_component = new_game.manager.add_component::<VelocityComponent>(player);
        velocity_component.set_velocity(1, 1);

        new_game
    }

	pub fn render(&mut self, graphics: &mut Graphics) {

        // clear screen
        graphics.renderer.clear();
        
        // use renderer system
        self.map.render(graphics);
        self.render_system.render(graphics, &self.manager);

		// render on screen
        graphics.renderer.present();
	}

	pub fn update(&mut self) {
        
        // use movement system
        self.movement_system.update(&mut self.manager);

        self.manager.refresh();
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