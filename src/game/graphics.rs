use sdl2::Sdl;
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use super::texture_manager::TextureManager;

pub struct Graphics<'a> {
    pub sdl_context: &'a Sdl,
    pub renderer: &'a mut Canvas<Window>,
    pub texture_manager: TextureManager<'a>,
}