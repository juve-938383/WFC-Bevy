use bevy::prelude::*;

mod systems;
use systems::*;

pub mod components;

pub struct TilemapManagerPlugin;



impl Plugin for TilemapManagerPlugin{
    fn build(&self, app: &mut App){

        app
            .add_systems(Startup, startup)
            .add_systems(Update, update);
    }
}