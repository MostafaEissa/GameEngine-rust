extern crate sdl2;
extern crate game_engine;


fn main() {
	let (sdl_context, mut renderer, mut texture_creature) =  game_engine::init_sdl("GameEngine", 800, 600, false).unwrap();
	let mut game = game_engine::Game::new(&sdl_context, &mut renderer, &mut texture_creature);

	while game.running() {
		game.handle_events();
		game.update();
		game.render();
	}
}

