use sdl2::render::{Texture, TextureCreator};
use sdl2::video::{WindowContext};
use sdl2::image::LoadTexture;
use std::collections::HashMap;

pub struct TextureManager {
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<String, Texture>
}

impl TextureManager {

    pub fn new(texture_creator:  TextureCreator<WindowContext>) -> Self {
        TextureManager{texture_creator: texture_creator, textures: HashMap::new()}
    }

    pub fn load(&mut self, path: &str) -> &Texture{
        if self.textures.contains_key(path) {
            return self.textures.get(path).unwrap();
        }

        let texture = self.texture_creator.load_texture(path).unwrap();
        self.textures.insert(path.to_string(), texture);
        self.textures.get(path).unwrap()
    }
}