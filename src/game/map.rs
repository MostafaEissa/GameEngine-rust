use std::fs::File;
use std::io::prelude::*;


pub struct Tile {
    pub texture_sheet: &'static str,
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32
}

pub struct Map;

impl Map {

    pub fn load_map(path: &str, size_x: usize, size_y: usize) -> Vec<Tile> {
        
        let mut map: Vec<Tile> = vec![];
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let rows: Vec<_> = contents.lines().collect();
        for row in 0..size_y {
                let tiles: Vec<_> = rows[row].split(',').collect();
                for column in 0..size_x {
                    let tile = tiles[column].chars().next().unwrap();
                    match tile {
                        '0' => {
                            map.push(Tile {
                                texture_sheet: "assets/water.png",
                                width: 32,
                                height: 32,
                                x: column as u32 * 32,
                                y: row as u32* 32
                            });
                        },
                        '1' => {
                            map.push(Tile {
                                texture_sheet: "assets/grass.png",
                                width: 32,
                                height: 32,
                                x: column as u32 * 32,
                                y: row as u32* 32
                            });
                        },
                        '2' => {
                            map.push(Tile {
                                texture_sheet: "assets/dirt.png",
                                width: 32,
                                height: 32,
                                x: column as u32 * 32,
                                y: row as u32* 32
                            });
                        },
                        _ => {
                            panic!("unkown tile type");
                        }
                    }
                }
        }
        map
    }
}