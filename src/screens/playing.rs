use crate::game_data;
use bevy::prelude::*;
use game_data::*;

pub struct PlayingPlugin;

fn state() -> GameState {
    GameState::Playing
}

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(state()).with_system(setup.system()))
            .add_system_set(SystemSet::on_update(state()).with_system(update.system()))
            .add_system_set(SystemSet::on_exit(state()).with_system(exit.system()));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "Playing: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/Roboto/Roboto-Bold.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(state());

    // let start_texture_handle = asset_server.load("flappy-bird/SpaceToStart.png");

    // commands.
    //     // Start Screen
    //     .spawn_bundle(SpriteBundle {
    //         material: materials.add(start_texture_handle.into()),
    //         ..Default::default()
    //     })
}

fn update(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(GameState::Menu).unwrap();
    }

    // // Update Score
    // for (mut text, _) in q.iter_mut() {
    //     text.sections[1].value = format!("{:.2}", gamedata.score );
    // }
}

fn exit() {}
