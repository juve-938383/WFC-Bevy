use super::super::config::config::*;

use rand::Rng;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::prelude::*;
use crate::tilemap_manager::components::{Cell, TilemapLayout};


//Initiates the tilemap.
pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tilemap_layout: ResMut<TilemapLayout>,
) {

    commands.spawn(Camera2dBundle::default());
    tilemap_layout.initialize_tilemap();

    //Textures of squares shown on an empty map.
    let texture_handle1: Handle<Image> = asset_server.load("../assets/sprites/bg_dark.png");
    let texture_handle2: Handle<Image> = asset_server.load("../assets/sprites/bg_light.png");
    //Create a list of all available tile assets for later reference.
    let mut texture_handles: Vec<Handle<Image>> = vec!(texture_handle1, texture_handle2);
    for path in TILE_SPRITES{
        texture_handles.push(asset_server.load(path));
    }

    //The code below is from the official bevy_ecs_tilemap Github examples:
    //https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/basic.rs
    let map_size: TilemapSize = TilemapSize { x: MAP_WIDTH as u32, y: MAP_HEIGHT as u32 };


    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity: Entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage: TileStorage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos: TilePos = TilePos { x, y };
            let texture_index: usize = tilemap_layout.cells[(y * map_size.x + x) as usize].final_tile;
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    //If the cell has been collapsed (its tile_index is less than 80), set the tile_index of the tile to that of the cell plus 2 to make up for the addition of the blank squares.
                    //Otherwise, pick a blank square based on the position of the tile.
                    texture_index: if texture_index < 80 {TileTextureIndex(texture_index as u32 + 2)} else {TileTextureIndex((x+y) % 2)},
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Vector(texture_handles),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

//Updates the tilemap if there have been changes.
pub fn update(
    tile_storage_query: Query<&TileStorage>,
    mut tile_query: Query<&mut TileTextureIndex>,
    tilemap_layout: ResMut<TilemapLayout>,
    ) {
    let cells = &tilemap_layout.cells;
    for tile_storage in tile_storage_query.iter(){
        for j in 0..(tile_storage.size.y) as i32{
            for i in 0..(tile_storage.size.x) as i32 {
                let tile: Option<Entity> = tile_storage.get(&TilePos {x:i as u32, y:j as u32});
                if let Ok(mut tile_texture) = tile_query.get_mut(tile.unwrap()) {
                    let index: usize = (j * MAP_WIDTH + i) as usize;
                    let final_tile: u32 = cells[index].final_tile as u32;
                    if final_tile < 80 {
                        if tile_texture.0 != final_tile + 2 {
                            tile_texture.0 = final_tile + 2;
                        }
                    } else {
                        //Setting this each time is necessary as without it, resetting the tilemap layout would not clear the visible tilemap.
                        tile_texture.0 = ((cells[index].x + cells[index].y) % 2) as u32;
                    }
                }
            }
        }
    }
}


