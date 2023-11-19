use bevy::prelude::*;
use super::super::config::config::*;
use rand::Rng;

//Represents the current, globally accessible, tilemap layout.
#[derive(Resource)]
pub struct TilemapLayout{
    pub cells: Vec<Cell>,
    pub initiated: bool,
    pub completed: bool
}

impl Default for TilemapLayout{
    fn default() -> TilemapLayout {
        TilemapLayout{
            cells: vec![],
            initiated: false,
            completed: false
        }
    }
}

impl TilemapLayout{
    pub fn initialize_tilemap(&mut self){
        let map_size: i32 = MAP_WIDTH * MAP_HEIGHT;
        for i: i32 in 0..map_size{
            let x: i32 = i % MAP_WIDTH;
            let y: i32 = i / MAP_WIDTH;
            let cell = Cell{
                x,
                y,
                available_tiles: ALL_TILES.to_vec(),
                final_tile: usize::MAX,
                entropy: 80,
                checked: false,
            };
            self.cells.push(cell);
        }
        self.initiated = false;
        self.completed = false;

    }

    //Iterates through cells and returns the entropy value and index of the cell of lowest entropy.
    pub fn get_min_entropy(&self) -> (i32, usize){
        let mut min_entropy: i32 = 80;
        let mut min_index: usize = 0;
        for i: usize in 0..self.cells.len(){
            let cell_entropy: i32 = self.cells[i].entropy;
            let final_tile: usize = self.cells[i].final_tile;
            if cell_entropy < min_entropy {
                if final_tile == usize::MAX {
                    min_entropy = cell_entropy;
                    min_index = i;
                }
            }
        }
        (min_entropy, min_index)
    }
}

//Represents the unit of which the tilemap layout is built.
pub struct Cell{
    //Position in the tilemap layout.
    pub x: i32,
    pub y: i32,
    //Current available tiles before collapsing.
    pub available_tiles: Vec<usize>,
    //Index of the definitive final tile after collapsing.
    pub final_tile: usize,
    //Count of available tiles.
    pub entropy: i32,
    //Shows whether the tile has been checked during the propagation phase to avoid ping-pong recursion.
    //Also true if tile has been collapsed.
    pub checked: bool
}

impl Cell{
    pub fn update_entropy(&mut self){
        self.entropy = self.available_tiles.len() as i32;
    }

}