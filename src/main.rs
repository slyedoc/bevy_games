mod game_state;
mod screens;
mod board;
mod utils;

use game_state::*;
use screens::*;
use board::*;
use utils::*;

use bevy::prelude::*;
mod plugins;
use plugins::{
    window_config::*,
};

fn main() {
    App::build()

        .add_plugin(WindowConfigPlugin)
        .insert_resource(WindowDescriptor {
            title: "Patrick's Rust Heaven".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.4, 0.4)))
        .add_state(GameState::Menu)
        // Setup cemaras
        .add_startup_system(setup.system())

        // Setup Board
        .add_plugin(BoardPlugin)

         // Setup screens
        .add_plugin(MenuPlugin)
        .add_plugin(PlayingPlugin)
        
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // Add the game's entities to our world

    // cameras
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());
   //commands.spawn_bundle(UiCameraBundle::default());

    }
