#![allow(unused_imports)]
#![allow(dead_code)]

mod board;
mod screens;
mod utils;
mod game_data;

use board::*;
use game_data::{ * };
use screens::*;
use utils::*;

use bevy::prelude::*;
mod plugins;
use plugins::window_config::*;

use bevy_inspector_egui::*;
mod egui_demo;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)

        .add_plugin(WindowConfigPlugin)
        //.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Patrick's Rust Heaven".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.4, 0.4)))

        // Setup Board
        .insert_resource(BoardSize { x: 19, y: 19 })
        .add_plugin(BoardPlugin)

        // Setup screens
        .add_state(GameState::Menu)
        .add_plugin(MenuPlugin)
        .add_plugin(PlayingPlugin)

        // Rember window location for debugging
        .add_plugin(WindowConfigPlugin)
        //.add_plugin(InspectorPlugin::<egui_demo::Data>::new( ))
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}

fn setup(mut commands: Commands) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    //commands.spawn_bundle(UiCameraBundle::default());
}

fn update(
    keyboard_input: Res<Input<KeyCode>>,
    mut wip: ResMut<WorldInspectorParams>
) {
    if keyboard_input.just_pressed(KeyCode::F12) {

        wip.enabled = !wip.enabled;
    }
}
