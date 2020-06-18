mod game_engine;
mod graphics;
mod map;
mod texture_manager;

pub use game_engine::GameEngine;
pub use graphics::Graphics;
pub use texture_manager::TextureManager;
pub use map::Map;

use sdl2::Sdl;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;

pub fn init_sdl(title: &str, width: u32, height: u32, fullscreen: bool) 
    -> Result<(Sdl, Canvas<Window>, TextureCreator<WindowContext>, EventPump), String> {
    
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
    let renderer = window.into_canvas().build().map_err(|err| err.to_string())?;
    println!("Renderer created!...");
    
    // create texture creator
    let texture_creator = renderer.texture_creator();

    // create event pump
    let event_pump = sdl.event_pump()?;
    println!("Event pump created!...");

    Ok((sdl, renderer, texture_creator, event_pump))
}