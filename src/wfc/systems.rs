use bevy::prelude::*;
use crate::tilemap_manager::components::*;
use rand::Rng;
use crate::config::config::*;
use crate::tilemap_manager::TilemapManagerPlugin;

//Applies Wave Function Collapse algorithm every frame.
pub fn wave_function_collapse(
    mut tilemap_layout: ResMut<TilemapLayout>,
    keys: Res<Input<KeyCode>>,
){
    //Reset map on Spacebar press.
    if keys.just_pressed(KeyCode::Space){
        reset(&mut tilemap_layout);
    }

    if !tilemap_layout.completed {
        let min_entropy_index: (i32, usize) = tilemap_layout.get_min_entropy();
        if min_entropy_index.0 == 80 && tilemap_layout.initiated {
            tilemap_layout.completed = true;
            return;
        }
        if min_entropy_index.0 > 0 {
            //Randomly collapsing the first cell.
            if !tilemap_layout.initiated {
                tilemap_layout.initiated = true;
                let x: i32 = rand::thread_rng().gen_range(0..MAP_WIDTH);
                let y: i32 = rand::thread_rng().gen_range(0..MAP_HEIGHT);
                collapse((y * MAP_WIDTH + x) as usize, &mut tilemap_layout);
            }
            //Collapsing subsequent cells.
            else {
                collapse(min_entropy_index.1, &mut tilemap_layout);
            }

            //Reset the "checked" status for non-collapsed cells.
            for i in 0..tilemap_layout.cells.len() {
                let cell: &mut Cell = &mut tilemap_layout.cells[i];
                if cell.final_tile < 80 {
                    cell.checked = true;
                } else {
                    cell.checked = false;
                }
            }
        }
        //Reset the tilemap layout when the algorithm is stuck.
        else {
            reset(&mut tilemap_layout);
        }

    }
    //Uncomment this line for automatic map reset.
    //else{reset(&mut tilemap_layout);}

}

//Collapses a cell and initiates restriction propagation.
pub fn collapse(index: usize, tilemap_layout: &mut TilemapLayout){
    //Using weighted randomness to pick the tile.
    let cell: &mut Cell = &mut tilemap_layout.cells[index];
    let mut tile_weights: Vec<usize> = vec![];
    let mut weight_sum: isize = 0;
    for i in 0..cell.available_tiles.len(){
        let weight: usize = TILE_WEIGHTS[cell.available_tiles[i]];
        tile_weights.push(weight);
        weight_sum += weight as isize;
    }
    let mut random_number: isize = rand::thread_rng().gen_range(0..weight_sum);

    for i in 0..cell.available_tiles.len(){
        random_number -= TILE_WEIGHTS[cell.available_tiles[i]] as isize;
        if random_number > 0{
            continue;
        }
        cell.final_tile = cell.available_tiles[i];
        break;
    }

    cell.available_tiles.clear();
    cell.checked = true;
    cell.update_entropy();
    //Restricting the 4 direct neighbours.
    let tile_types: [usize; 4] = TILE_RULES[cell.final_tile];
    let x: i32 = cell.x;
    let y: i32 = cell.y;
    if x != 0 {
        restrict(index - 1, WEST, vec![tile_types[WEST]], tilemap_layout);
    }
    if x != MAP_WIDTH - 1 {
        restrict(index + 1, EAST, vec![tile_types[EAST]], tilemap_layout);
    }
    if y != 0 {
        restrict(index - MAP_WIDTH as usize, SOUTH, vec![tile_types[SOUTH]], tilemap_layout);
    }
    if y != MAP_HEIGHT - 1 {
        restrict(index + MAP_WIDTH as usize, NORTH, vec![tile_types[NORTH]], tilemap_layout);

    }

}

//Recursively restricts the cells of the tilemap layout.
pub fn restrict(index: usize, direction: usize ,tile_type: Vec<usize>, tilemap_layout: &mut TilemapLayout){
    let opposite_direction: usize = OPPOSITE_DIRECTIONS[direction];
    let cell: &mut Cell = &mut tilemap_layout.cells[index];
    let tiles_vec: &mut Vec<usize> = &mut cell.available_tiles;
    let tiles_vec_copy: Vec<usize> = tiles_vec.to_vec();
    let length: usize = tiles_vec_copy.len();
    for i in 0..length {
        let mut remove: bool = true;
        let check_vec_len: usize = tile_type.len();
        let current_tile_type: usize = TILE_RULES[tiles_vec_copy[i]][opposite_direction];
        for j in 0..check_vec_len {
            if current_tile_type == tile_type[j] {
                remove = false;
                break;
            }
        }
        if remove {
            tiles_vec.retain(|&x| x != tiles_vec_copy[i]);
        }
    }
    cell.checked = true;
    cell.update_entropy();


    let x: i32 = cell.x;
    let y: i32 = cell.y;
    if cell.entropy < 80 && cell.entropy > 0{
        let mut tile_types_north: Vec<usize> = vec![];
        let mut tile_types_east: Vec<usize> = vec![];
        let mut tile_types_south: Vec<usize> = vec![];
        let mut tile_types_west: Vec<usize> = vec![];
        for i in 0..cell.available_tiles.len() {
            let tile_sides: [usize; 4] = TILE_RULES[cell.available_tiles[i]];
            tile_types_north.push(tile_sides[NORTH]);
            tile_types_east.push(tile_sides[EAST]);
            tile_types_south.push(tile_sides[SOUTH]);
            tile_types_west.push(tile_sides[WEST]);
        }

        if x != 0 {
            if !tilemap_layout.cells[index - 1].checked {
                restrict(index - 1, WEST, tile_types_west, tilemap_layout);
            }
        }
        if x != (MAP_WIDTH - 1) {
            if !tilemap_layout.cells[index + 1].checked {
                restrict(index + 1, EAST, tile_types_east, tilemap_layout);
            }
        }
        if y != 0 {
            if !tilemap_layout.cells[index - MAP_WIDTH as usize].checked {
                restrict(index - MAP_WIDTH as usize, SOUTH, tile_types_south, tilemap_layout);
            }
        }
        if y != (MAP_HEIGHT - 1) {
            if !tilemap_layout.cells[index + MAP_WIDTH as usize].checked {
                restrict(index + MAP_WIDTH as usize, NORTH, tile_types_north, tilemap_layout);
            }
        }
    }
}

//Resets the tilemap layout.
pub fn reset(tilemap_layout: &mut TilemapLayout){
    tilemap_layout.cells.clear();
    tilemap_layout.initialize_tilemap();
}

