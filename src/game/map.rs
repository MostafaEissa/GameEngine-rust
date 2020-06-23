use std::fs::File;
use std::io::prelude::*;


pub struct Tile {
    pub texture_sheet: &'static str,
    pub src_rect: sdl2::rect::Rect,
    pub pos_x: u32,
    pub pos_y: u32
}


pub struct Map;

impl Map {

    pub fn load_map(path: &str, size_x: usize, size_y: usize) -> (Vec<Tile>, Vec<Tile>) {
        
        let mut map: Vec<Tile> = vec![];
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let rows: Vec<_> = contents.lines().collect();
        for row in 0..size_y {
                let tiles: Vec<_> = rows[row].split(',').collect();
                for column in 0..size_x {
                    let position = tiles[column].parse::<i32>().unwrap();
                    let y = position / 10;
                    let x = position % 10;
                    map.push(Tile {
                        texture_sheet: "assets/terrain_ss.png",
                        src_rect: sdl2::rect::Rect::new(x * 32, y * 32, 32, 32),
                        pos_x: column as u32 * 64,
                        pos_y: row as u32* 64
                    });
                }
        }

        let mut colliders = vec![];
        let rows = &rows[size_y+1..];
        for row in 0..size_y {
            let tiles: Vec<_> = rows[row].split(',').collect();
            for column in 0..size_x {
                let position = tiles[column].parse::<i32>().unwrap();
                if position == 1 {
                    colliders.push (
                        Tile {
                            texture_sheet: "",
                            src_rect: sdl2::rect::Rect::new(0, 0, 32, 32),
                            pos_x: column as u32 * 64,
                            pos_y: row as u32* 64
                        }
                    );
                }
            }
        }

        (map, colliders)
    }
}