extern crate sdl2;
extern crate game_engine;


fn main() {
	let mut game = game_engine::Game::init("GameEngine", 800, 600, false).unwrap();

	game.start(60);
}

