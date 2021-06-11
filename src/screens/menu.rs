use crate::game_data;
use crate::InspectorPlugin;
use bevy::{app::AppExit, prelude::*};
use bevy_inspector_egui::*;
use game_data::*;

pub struct MenuPlugin;

fn state() -> GameState {
    GameState::Menu
}

#[derive(Inspectable)]
pub enum TextColor {
    White,
    Green,
    Blue,
}

#[derive(Inspectable)]
pub struct MenuData {
    #[inspectable(min = 1, max = 22)]
    board_x: u8,
    #[inspectable(min = 1, max = 22)]
    board_y: u8,
    name: String,
    color: Color,
    enabled: bool,
}

impl Default for MenuData {
    fn default() -> Self {
        MenuData {
            board_x: 9,
            board_y: 9,
            name: "".to_string(),
            color: Color::BLUE,
            enabled: true
        }
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InspectorPlugin::<MenuData>::new())
            //.add_system_set(SystemSet::on_enter(state()).with_system(setup.system()))
            .add_system_set(SystemSet::on_update(state()).with_system(update.system()));
    }
}

fn update(
    keyboard_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}