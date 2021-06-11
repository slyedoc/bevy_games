use crate::game_state;
use bevy::prelude::*;
use game_state::*;


pub struct TempPlugin;
let state = GameState::Menu;

impl Plugin for TempPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let state = GameState::Menu;
        app
        .add_system_set(SystemSet::on_enter(state).with_system(setup.system()))
        .add_system_set(SystemSet::on_update(state).with_system(update.system()))
        .add_system_set(SystemSet::on_exit(state).with_system(exit.system()));
    }
}

fn setup() {

}

fn update() {

}

fn exit() {}