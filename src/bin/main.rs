extern crate sdl2;
extern crate game_engine;

use game_engine::ecs::World;
use game_engine::ecs::component::*;
use game_engine::ecs::system::{RenderSystem, AnimationSystem, PhysicsSystem, KeyboardSystem};
use game_engine::math::{Vector2D};

fn main() {
	let (sdl_context, renderer, texture_creator, event_pump) =  game_engine::game::init_sdl("GameEngine", 800, 640, false).unwrap();

	let mut world = World::new();
	let mut dispatcher = game_engine::GameEngine::new();

	const MAP_LAYER: u32 = 0;
	const PLAYER_LAYER: u32 = 1;
	const ENEMY_LAYER: u32 = 2;

	// create player
	let player = world.create_entity();
	world.add_component::<SpriteComponent>(player)
	.set_texture("assets/player_anims.png")
	.set_layer(PLAYER_LAYER)
	.set_region(Rect::new(32, 32))
	.set_scale(2, 2);

	world.add_component::<PositionComponent>(player).set_position(Vector2D::new(0.0, 300.0));
	world.add_component::<VelocityComponent>(player).set_velocity(3.0, Vector2D::new(0.0, 0.0));
	world.add_component::<KeyboardControlled>(player);
	world.add_component::<CollisionComponent>(player).set_width(32).set_height(32).set_tag("player");

	world.add_component::<AnimationComponent>(player)
	.add_animation("Idle", 0, 4, 100)
	.add_animation("Walk", 1, 8, 100)
	.play("Idle", false, false);

	//create enemy
	let enemy = world.create_entity();
	world.add_component::<SpriteComponent>(enemy).set_texture("assets/enemy.png").set_layer(ENEMY_LAYER).set_region(Rect::new(32, 32)).set_scale(2, 2);
	world.add_component::<PositionComponent>(enemy).set_position(Vector2D::new(50.0, 50.0));
	world.add_component::<VelocityComponent>(enemy).set_velocity(3.0, Vector2D::new(0.0, 0.0));
	world.add_component::<CollisionComponent>(enemy).set_width(32).set_height(32).set_tag("enemy");

	for tile in game_engine::game::Map::load_map("assets/p25x20.map", 25, 20) {
		let tile_entity = world.create_entity();
		world.add_component::<SpriteComponent>(tile_entity).set_texture(tile.texture_sheet).set_layer(MAP_LAYER).set_region(Rect::new(tile.width, tile.height)).set_scale(1, 1);
		world.add_component::<PositionComponent>(tile_entity).set_position(Vector2D::new(tile.x as f32, tile.y as f32));
		world.add_component::<VelocityComponent>(tile_entity);
	}
	
	// keyboard resource
	world.add_resource::<KeyboardComponent>();
	world.add_resource::<Ticks>();

	// initialize systems
	let render_system = RenderSystem::new(sdl_context, renderer, texture_creator);

	
	dispatcher.register(KeyboardSystem);
	dispatcher.register(PhysicsSystem);
	dispatcher.register(AnimationSystem);
	dispatcher.register(render_system);
	
	dispatcher.run(&mut world, event_pump);
}

