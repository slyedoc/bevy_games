use bevy::prelude::*;
mod game;
use game::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    GameOver,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Board::new(Player::O))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

/// Setup System
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Setup camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Build the board
    const BOARD_SPACING: f32 = 10.;
    const BOARD_SIZE: f32 = 80.;
    const BOARD_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
    const BOARD_DROPSHADOW: (f32, f32) = (20.0, 30.0);
    const BOARD_DROPSHADOW_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
    for x in -1..2 {
        let x2 = x as f32;
        for y in -1..2 {
            let y2 = y as f32;
            let spacing = BOARD_SIZE + BOARD_SPACING;

            // Draw top board locations
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(BOARD_COLOR.into()),
                transform: Transform::from_xyz(spacing * x2, spacing * y2, 0.0),
                sprite: Sprite::new(Vec2::new(BOARD_SIZE, BOARD_SIZE)),
                ..Default::default()
            });

            // draw drop shadow
            println!(
                "spacing: {}, withoffset: {}",
                (spacing * x2),
                (spacing * x2) + BOARD_DROPSHADOW.0
            );
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(BOARD_DROPSHADOW_COLOR.into()),
                transform: Transform::from_xyz(
                    (spacing * x2) + BOARD_DROPSHADOW.0,
                    (spacing * y2) + BOARD_DROPSHADOW.1,
                    -10.0,
                ),
                sprite: Sprite::new(Vec2::new(BOARD_SIZE, BOARD_SIZE)),
                ..Default::default()
            });
        }
    }
    println!("");
}
// board
