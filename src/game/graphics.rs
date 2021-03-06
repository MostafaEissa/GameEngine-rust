use sdl2::Sdl;
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use super::texture_manager::TextureManager;

pub struct Graphics {
    pub sdl_context: Sdl,
    pub renderer: Canvas<Window>,
    pub texture_manager: TextureManager,
}