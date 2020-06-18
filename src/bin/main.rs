extern crate sdl2;
extern crate game_engine;
use game_engine::ecs::World;
use game_engine::ecs::{TransformComponent, TextureComponent, VelocityComponent, Rect};
use game_engine::ecs::{RenderSystem, MovementSystem};
use game_engine::math::Vector2D;

fn main() {
	let (sdl_context, renderer, texture_creator, event_pump) =  game_engine::game::init_sdl("GameEngine", 800, 640, false).unwrap();

	let mut world = World::new();
	let mut dispatcher = game_engine::GameEngine::new();

	// create player
	let player = world.create_entity();
	world.add_component::<TextureComponent>(player).set_texture("assets/player.png", Rect {width: 32, height: 32}, Rect {width: 64, height: 64});
	world.add_component::<TransformComponent>(player).set_position(Vector2D::new(0.0, 300.0));
	world.add_component::<VelocityComponent>(player).set_velocity(1, 1);

	//create enemy
	let enemy = world.create_entity();
	world.add_component::<TextureComponent>(enemy).set_texture("assets/enemy.png", Rect {width: 32, height: 32}, Rect {width: 64, height: 64});
	world.add_component::<TransformComponent>(enemy).set_position(Vector2D::new(150.0, 150.0));
	//world.add_component::<VelocityComponent>(enemy).set_velocity(-1, 0);

	// initialize systems
	let render_system = RenderSystem::new(sdl_context, renderer, texture_creator);
	let movement_system = MovementSystem::new();

	dispatcher.register(render_system);
	dispatcher.register(movement_system);
	dispatcher.run(&mut world, event_pump);
}

