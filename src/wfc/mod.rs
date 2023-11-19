use bevy::prelude::*;

mod systems;
use systems::*;

pub struct WFCPlugin;

impl Plugin for WFCPlugin{
    fn build(&self, app: &mut App){

        app
            .add_systems(Update, wave_function_collapse);
    }
}