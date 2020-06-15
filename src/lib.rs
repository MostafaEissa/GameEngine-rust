mod game;
pub use game::Game;

use sdl2::Sdl;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub fn init_sdl(title: &str, width: u32, height: u32, fullscreen: bool) 
-> Result<(Sdl, Canvas<Window>, TextureCreator<WindowContext>), String> {
    
    // initialize SDL
    let sdl = sdl2::init()?;
    
    println!("Subsystems initialized!...");
    
    // create window
    let video_subsystem = sdl.video()?;
    let mut window = video_subsystem.window(title, width, height);
    
    //set position
    window.position_centered();

    // apply fullscreen
    if fullscreen {
        window.fullscreen();
    } 

    let window = window.build().map_err(|err| err.to_string())?;
    println!("Window created!...");
    
    // create renderer
    let mut renderer = window.into_canvas().build().map_err(|err| err.to_string())?;
    println!("Renderer created!...");
    
    // create texture creator
    let texture_creator = renderer.texture_creator();

    // set drawing color
    renderer.set_draw_color(sdl2::pixels::Color::WHITE);

    Ok((sdl, renderer, texture_creator))
}