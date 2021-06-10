use crate::bird;
use crate::gamedata;
use crate::physics;
use crate::screens;
use bevy::prelude::*;

use bird::*;
use gamedata::*;
use physics::*;
use screens::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
    Dead,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Menu)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()))
        .add_system(handle_gamestate_system.system())
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()));
    }
}

fn handle_gamestate_system(
    mut game_data: ResMut<GameData>,
    keyboard_input: Res<Input<KeyCode>>,
    mut q: QuerySet<(
        Query<(&Player, &mut Transform, &mut Velocity)>, //player_query
        Query<(&EndScreen, &mut Visible)>, //q1 - end_screen_query
        Query<(&StartScreen, &mut Visible)>, //q2 - start_screen
    )>,
) {
    println!("{:?}", game_data.game_state);

    match game_data.game_state {

        GameState::Menu => {
            if keyboard_input.pressed(KeyCode::Space) {
                game_data.game_state = GameState::Playing;
                for (_ss, mut vis) in &mut q.q2_mut().iter_mut() {
                    //start_screen_query
                    vis.is_visible = false;
                }
                for (_ss, mut vis) in &mut q.q1_mut().iter_mut() {
                    //start_screen_query
                    vis.is_visible = true;
                }
            }
        }
        GameState::Playing => {}
        GameState::Dead => {              
            if keyboard_input.pressed(KeyCode::Space) {

                game_data.game_state = GameState::Playing;
                println!("{:?}", game_data.game_state);
                for (_p, mut trans, mut velocity) in &mut q.q0_mut().iter_mut() {
                    trans.translation = Vec3::new(0.0, 0.0, 100.0);
                    velocity.0.y = 0.0;
                }
                for (_ss, mut vis) in &mut q.q2_mut().iter_mut() {
                    //start_screen_query
                    vis.is_visible = true;
                }
                for (_es, mut vis) in &mut q.q1_mut().iter_mut() {
                    //end_screen_query
                    vis.is_visible = false;
                }
            }
        }
    }


}

