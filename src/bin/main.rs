extern crate sdl2;
extern crate game_engine;


fn main() {
	let (sdl_context, mut renderer, texture_creator) =  game_engine::init_sdl("GameEngine", 800, 640, false).unwrap();
	let mut game = game_engine::Game::new(&sdl_context, &mut renderer, & texture_creator);

	while game.running() {
		game.handle_events();
		game.update();
		game.render();
	}
}

