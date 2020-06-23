extern crate sdl2;
extern crate game_engine;

use game_engine::ecs::World;
use game_engine::ecs::component::*;
use game_engine::ecs::system::{RenderSystem, AnimationSystem, PhysicsSystem, KeyboardSystem};
use game_engine::math::{Vector2D};
use sdl2::rect::Rect;

fn main() {
	let (sdl_context, renderer, texture_creator, event_pump) =  game_engine::game::init_sdl("GameEngine", 800, 640, false).unwrap();

	let mut world = World::new();
	let mut dispatcher = game_engine::GameEngine::new();

	const MAP_LAYER: u32 = 0;
	const PLAYER_LAYER: u32 = 1;

	// create player
	let player = world.create_entity();
	world.add_component::<SpriteComponent>(player)
	.set_texture("assets/player_anims.png")
	.set_layer(PLAYER_LAYER)
	.set_region(Rect::new(0, 0, 32, 32))
	.set_scale(4, 4);

	world.add_component::<PositionComponent>(player).set_position(Vector2D::new(400.0, 320.0));
	world.add_component::<VelocityComponent>(player).set_velocity(3.0, Vector2D::new(0.0, 0.0));
	world.add_component::<KeyboardControlled>(player);
	world.add_component::<CollisionComponent>(player).set_width(32).set_height(32).set_tag("player");

	world.add_component::<AnimationComponent>(player)
	.add_animation("Idle", 0, 4, 100)
	.add_animation("Walk", 1, 8, 100)
	.play("Idle", false, false);

	let (tiles, terrain) =  game_engine::game::Map::load_map("assets/map.map", 25, 20);

	for tile in tiles {
		let tile_entity = world.create_entity();
		world.add_component::<SpriteComponent>(tile_entity).set_texture(tile.texture_sheet).set_layer(MAP_LAYER).set_region(tile.src_rect).set_scale(2, 2);
		world.add_component::<PositionComponent>(tile_entity).set_position(Vector2D::new(tile.pos_x as f32, tile.pos_y as f32));
		world.add_component::<VelocityComponent>(tile_entity);
	}

	for tile in terrain {
		let tile_entity = world.create_entity();
		world.add_component::<PositionComponent>(tile_entity).set_position(Vector2D::new(tile.pos_x as f32, tile.pos_y as f32));
		world.add_component::<VelocityComponent>(tile_entity);
		world.add_component::<CollisionComponent>(tile_entity).set_width(tile.src_rect.w as u32).set_height(tile.src_rect.h as u32).set_tag("terrain");
	}
	
	// keyboard resource
	world.add_resource::<KeyboardComponent>();
	world.add_resource::<Ticks>();
	world.add_resource::<Camera>();

	// initialize systems
	let render_system = RenderSystem::new(sdl_context, renderer, texture_creator);

	
	dispatcher.register(KeyboardSystem);
	dispatcher.register(PhysicsSystem);
	dispatcher.register(AnimationSystem);
	dispatcher.register(render_system);
	
	dispatcher.run(&mut world, event_pump);
}

