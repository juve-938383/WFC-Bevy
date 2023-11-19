#![allow(dead_code)]
#![allow(unused_imports)]

use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use bevy_ecs_tilemap::prelude::*;

pub mod tilemap_manager;
use tilemap_manager::*;

pub mod wfc;
use wfc::WFCPlugin;

pub mod config;
use config::*;
use crate::tilemap_manager::components::TilemapLayout;


fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from("Wave Function Collapse"),
                mode: WindowMode::Windowed,

                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_plugins(TilemapManagerPlugin)
        .add_plugins(WFCPlugin)
        .init_resource::<TilemapLayout>()
        .run();
}

