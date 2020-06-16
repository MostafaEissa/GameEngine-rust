use super::graphics::Graphics;
use sdl2::rect::Rect; 

enum Tile {
    Dirt,
    Grass,
    Water
}

pub struct Map {
    map: [[Tile; 25]; 20],
}

impl Map {

    pub fn load_map() -> Self {
        use Tile::*;
        let lvl1 = [
            [
            Water, Water, Water, Grass, Grass, Grass, Grass, Grass, Grass, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Grass, Grass, Grass, Dirt, Dirt, Dirt, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Grass, Grass, Grass, Dirt, Dirt, Dirt, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Grass, Grass, Dirt, Dirt, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Grass, Grass, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
            [
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water,
            Water, Water, Water, Water, Water, Water, Water, Water, Water, Water, 
            Water, Water, Water, Water, Water
            ],
        ];

        Map {map: lvl1}
    }

    pub fn render(&self, graphics: &mut Graphics) {
        let src = Rect::new(0, 0, 32, 32);
        let mut dest = Rect::new(0, 0, 32, 32);

        for row in 0..20 {
            for column in 0..25 {
                dest.set_x((column as u32 * dest.width()) as i32);
                dest.set_y((row as u32 * dest.height()) as i32);

                match self.map[row][column] {
                    Tile::Water => Self::draw_tile(graphics, "assets/water.png", src, dest),
                    Tile::Dirt => Self::draw_tile(graphics, "assets/dirt.png", src, dest),
                    Tile::Grass => Self::draw_tile(graphics, "assets/grass.png", src, dest),
                }
            }
        }
    }

    fn draw_tile(graphics: &mut Graphics, path: &str, src: Rect, dest: Rect) {
        let texture  = graphics.texture_manager.load(path);
        graphics.renderer.copy(texture, src, dest).unwrap();
    }
}