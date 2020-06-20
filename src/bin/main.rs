extern crate sdl2;
extern crate game_engine;

use game_engine::ecs::World;
use game_engine::ecs::component::*;
use game_engine::ecs::system::{RenderSystem, PhysicsSystem, KeyboardSystem};
use game_engine::math::{Vector2D};

fn main() {
	let (sdl_context, renderer, texture_creator, event_pump) =  game_engine::game::init_sdl("GameEngine", 800, 640, false).unwrap();

	let mut world = World::new();
	let mut dispatcher = game_engine::GameEngine::new();

	// create player
	let player = world.create_entity();
	world.add_component::<SpriteComponent>(player).set_texture("assets/player.png").set_region(Rect::new(32, 32)).set_scale(2, 2);
	world.add_component::<PositionComponent>(player).set_position(Vector2D::new(0.0, 300.0));
	world.add_component::<VelocityComponent>(player).set_velocity(3.0, Vector2D::new(0.0, 0.0));
	world.add_component::<KeyboardControlled>(player);
	world.add_component::<CollisionComponent>(player).set_width(32).set_height(32).set_tag("player");

	//create enemy
	let enemy = world.create_entity();
	world.add_component::<SpriteComponent>(enemy).set_texture("assets/enemy.png").set_region(Rect::new(32, 32)).set_scale(2, 2);
	world.add_component::<PositionComponent>(enemy).set_position(Vector2D::new(50.0, 50.0));
	world.add_component::<VelocityComponent>(enemy).set_velocity(3.0, Vector2D::new(0.0, 0.0));
	world.add_component::<CollisionComponent>(enemy).set_width(32).set_height(32).set_tag("enemy");

	for tile in game_engine::game::Map::load_map("assets/p25x20.map", 25, 20) {
		let tile_entity = world.create_entity();
		world.add_component::<SpriteComponent>(tile_entity).set_texture(tile.texture_sheet).set_region(Rect::new(tile.width, tile.height)).set_scale(1, 1);
		world.add_component::<PositionComponent>(tile_entity).set_position(Vector2D::new(tile.x as f32, tile.y as f32));
		world.add_component::<VelocityComponent>(tile_entity);
	}
	// water tile
	let water = world.create_entity();
	world.add_component::<SpriteComponent>(water).set_texture("assets/water.png").set_region(Rect::new(32, 32)).set_scale(1, 1);
	world.add_component::<PositionComponent>(water).set_position(Vector2D::new(200.0, 200.0));
	world.add_component::<VelocityComponent>(water);

	// dirt tile
	let dirt = world.create_entity();
	world.add_component::<SpriteComponent>(dirt).set_texture("assets/dirt.png").set_region(Rect::new(32, 32)).set_scale(1, 1);
	world.add_component::<PositionComponent>(dirt).set_position(Vector2D::new(250.0, 250.0));
	world.add_component::<VelocityComponent>(dirt);
	world.add_component::<CollisionComponent>(dirt).set_width(32).set_height(32).set_tag("dirt");

	// grass tile
	let grass = world.create_entity();
	world.add_component::<SpriteComponent>(grass).set_texture("assets/grass.png").set_region(Rect::new(32, 32)).set_scale(1, 1);
	world.add_component::<PositionComponent>(grass).set_position(Vector2D::new(150.0, 150.0));
	world.add_component::<VelocityComponent>(grass);
	world.add_component::<CollisionComponent>(grass).set_width(32).set_height(32).set_tag("grass");
	
	// keyboard resource
	world.add_resource::<KeyboardComponent>();

	// initialize systems
	let render_system = RenderSystem::new(sdl_context, renderer, texture_creator);

	
	dispatcher.register(KeyboardSystem);
	dispatcher.register(PhysicsSystem);
	dispatcher.register(render_system);
	
	dispatcher.run(&mut world, event_pump);
}

