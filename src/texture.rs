use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::{WindowContext};

pub struct TextureManager<'a> {
    texture_creator: &'a TextureCreator<WindowContext>
}

impl<'a> TextureManager<'a> {

    pub fn new(texture_creator:  &'a TextureCreator<WindowContext>) -> Self {
        TextureManager{texture_creator}
    }

    pub fn load_texture(&'a self, path: &str) -> Texture<'a>{
        self.texture_creator.load_texture(path).unwrap()
    }
}
