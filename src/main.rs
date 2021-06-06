use bevy::{core::FixedTimestep, prelude::*};
mod plugins;
use plugins::*;



const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(window_config::WindowConfigPlugin)
        .insert_resource(WindowDescriptor {
            title: "Patrick Rust Heaven".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.4, 0.4)))
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                // .with_system(paddle_movement_system.system())
                // .with_system(ball_collision_system.system())
                // .with_system(ball_movement_system.system()),
        )
        .run();
}

struct Board;


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // board
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.1, 1.0).into()),
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .insert(Board);

    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/Roboto/Roboto-Regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/Roboto/Roboto-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}
