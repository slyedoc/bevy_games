use bevy::prelude::*;
mod animation;
mod bird;
mod bounds_deletion;
mod clouds;
mod game_state;
mod gamedata;
mod mountains;
mod physics;
mod pipes;
mod screens;

use animation::*;
use bird::*;
use clouds::*;
use game_state::*;
use gamedata::*;
use mountains::*;
use physics::*;
use pipes::*;
use screens::*;

fn main() {
    App::build()
        .insert_resource(GameData { score: 0 })
        .insert_resource(ClearColor(Color::rgb(0.34, 0.75, 0.79)))
        .insert_resource(JumpHeight(23.0 * 40.0))
        .insert_resource(Gravity(45.0 * 40.0))
        .add_state(GameState::Menu)
        .add_startup_system(setup.system())
        // Menu
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu_input.system()))
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup.system()))
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(menu_exit.system()))

        // Playing
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(playing_input.system()))
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(playing_setup.system()))
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(playing_exit.system()))

        // Dead - Game Over
        .add_system_set(SystemSet::on_update(GameState::Dead).with_system(dead_input.system()))
        .add_system_set(SystemSet::on_enter(GameState::Dead).with_system(dead_setup.system()))
        .add_system_set(SystemSet::on_exit(GameState::Dead).with_system(dead_exit.system()))

        .add_plugin(PipePlugin)
        .add_plugin(BirdPlugin)
        .add_plugin(CloudPlugin)
        .add_plugin(MountainPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d(  ));
    commands.spawn_bundle(UiCameraBundle::default());
}

fn menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let start_texture_handle = asset_server.load("flappy-bird/SpaceToStart.png");

    commands
        // Start Screen
        .spawn_bundle(SpriteBundle {
            material: materials.add(start_texture_handle.into()),
            ..Default::default()
        })
        .insert(MenuScreen);
}

fn menu_input(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
    }
}

fn menu_exit(mut commands: Commands, mut q: Query<(Entity, With<MenuScreen>)>) {
    for (e, _) in q.iter_mut() {
        commands.entity(e).despawn();
    }
}

fn playing_setup(
    mut commands: Commands,
    mut gamedata: ResMut<GameData>,
    asset_server: Res<AssetServer>,
) {

    gamedata.score = 0;

    commands.spawn_bundle(TextBundle {
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
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/Roboto/Roboto-Bold.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/Roboto/Roboto-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::YELLOW,
                    },
                }
            ],
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(PlayingScreen);

}

fn playing_input(
    keyboard_input: Res<Input<KeyCode>>,
    gamedata: Res<GameData>,
    mut state: ResMut<State<GameState>>,
    mut q: Query<(&mut Text, With<PlayingScreen> )>,
) {

    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(GameState::Menu).unwrap();
    }

    // Update Score
    for (mut text, _) in q.iter_mut() {
        text.sections[1].value = format!("{:.2}", gamedata.score );
    }
}

fn playing_exit(mut commands: Commands, mut q: Query<(Entity, With<PlayingScreen>)>) {
    for (e, _) in q.iter_mut() {
        commands.entity(e).despawn();
    }
}

fn dead_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gamedata: Res<GameData>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let game_over_texture_handle = asset_server.load("flappy-bird/GameOverText.png");

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(game_over_texture_handle.into()),
            draw: Draw {
                render_commands: Vec::new(),
            },
            ..Default::default()
        })
        .insert(DeadScreen);

        commands.spawn_bundle(TextBundle {
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
                sections: vec![
                    TextSection {
                        value: "Final Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Roboto/Roboto-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: gamedata.score.to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Roboto/Roboto-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::RED,
                        },
                    }
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DeadScreen);
}

fn dead_input(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
    }
}

fn dead_exit(mut commands: Commands, mut q: Query<(Entity, With<DeadScreen>)>) {
    for (e, _) in q.iter_mut() {
        commands.entity(e).despawn();
    }
}
