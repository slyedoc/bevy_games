use bevy::prelude::*;
mod animation;
mod bird;
mod bounds_deletion;
mod clouds;
mod gamedata;
mod gamestate;
mod mountains;
mod physics;
mod pipes;
mod screens;

use animation::*;
use bird::*;
use clouds::*;
use gamedata::*;
use gamestate::*;
use mountains::*;
use physics::*;
use pipes::*;
use screens::*;

fn main() {
    App::build()
        .insert_resource(GameData {
            game_state: GameState::Menu,
            score: 0,
        })
        .insert_resource(ClearColor(Color::rgb(0.34, 0.75, 0.79)))
        .insert_resource(JumpHeight(23.0 * 40.0))
        .insert_resource(Gravity(45.0 * 40.0))
        .add_startup_system(setup.system())
        .add_plugin(PipePlugin)
        .add_plugin(BirdPlugin)
        .add_plugin(CloudPlugin)
        .add_plugin(MountainPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(ScreensPlugin)
        .add_plugin(GameStatePlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
    Dead,
}

pub struct StartScreen;
pub struct EndScreen;



fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
