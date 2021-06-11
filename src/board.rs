use crate::game_data;
use crate::utils;
use bevy::{math::*, prelude::*};
use bevy_inspector_egui::Inspectable;

use game_data::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<BoardClearEvent>()
            .add_system(clear.system())
            .add_event::<BoardSpawnEvent>()
            .add_system(spawn_board.system())
            .add_startup_system(setup.system());
        // only runs on
    }
}

pub struct BoardClearEvent;
pub struct BoardSpawnEvent;

fn clear(
    mut commands: Commands,
    //windows: Res<Windows>,
    mut clear: EventReader<BoardClearEvent>,
    mut q: Query<(Entity, With<BoardPart>)>,
) {
    // did anyone say clear?
    for r in clear.iter() {
        // destory existing board everything
        for (e, _) in q.iter_mut() {
            commands.entity(e).despawn();
        }
    }
}

#[derive(Inspectable, Default)]
pub struct BoardCell {
    pub x: u8,
    pub y: u8,
}

enum CellState {
    Closed(Player),
}

#[derive(Inspectable)]
enum BoardPart {
    Background,
    Cell,
}

fn spawn_board(
    mut commands: Commands,
    //windows: Res<Windows>,
    board_size: Res<BoardSize>,
    mut spawn: EventReader<BoardSpawnEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // did anyone say clear?
    for _ in spawn.iter() {
        println!("spawn_board");
        let aspect = board_size.x as f32 / board_size.y as f32;
        let aspect_scale = 700.;
        let size = Vec2::new(aspect * aspect_scale, aspect_scale); // this is size in world space, not including padding around the outside

        let cell_size = Vec2::new(
            -size.x / board_size.x as f32 / 2.,
            size.y / board_size.y as f32 / 2.,
        );
        let background_size = Vec2::new(-size.x + cell_size.x * 2., size.y + cell_size.y * 2.);
        let board_trim: f32 = 1.1;

        // board-trim
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::BLACK.into()),
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::ONE,
                },
                sprite: Sprite {
                    size: background_size * board_trim,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(BoardPart::Background);

        // board
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::SILVER.into()),
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::ONE,
                },
                sprite: Sprite {
                    size: background_size,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(BoardPart::Background);

        // draw board line
        let thinkness = 2.;
        for x in 0..board_size.x {
            let x = utils::map_range(
                x as f32,
                0.0,
                (board_size.x - 1) as f32,
                -size.x / 2.,
                size.x / 2.,
            );
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::BLACK.into()),
                    transform: Transform {
                        translation: Vec3::new(x, 0., 0.),
                        rotation: Quat::IDENTITY,
                        scale: Vec3::ONE,
                    },
                    sprite: Sprite {
                        size: Vec2::new(thinkness, size.y),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(BoardPart::Background);
        }
        for y in 0..board_size.y {
            let y = utils::map_range(
                y as f32,
                0.0,
                (board_size.y - 1) as f32,
                -size.y / 2.,
                size.y / 2.,
            );
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::BLACK.into()),
                    transform: Transform {
                        translation: Vec3::new(0., y, 0.),
                        rotation: Quat::IDENTITY,
                        scale: Vec3::ONE,
                    },
                    sprite: Sprite {
                        size: Vec2::new(size.x, thinkness),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(BoardPart::Background);
        }

        // Setup the board positions
        let cell_scale: f32 = 1.;
        for cell_x in 0..board_size.x {
            for cell_y in 0..board_size.y {
                // Find cell positions
                let x = utils::map_range(
                    cell_x as f32,
                    0.0,
                    (board_size.x - 1) as f32,
                    -size.x / 2.,
                    size.x / 2.,
                );
                let y = utils::map_range(
                    cell_y as f32,
                    0.0,
                    (board_size.y - 1) as f32,
                    -size.y / 2.,
                    size.y / 2.,
                );

                // Create board peice
                commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.add(Color::RED.into()),
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.),
                            rotation: Quat::IDENTITY,
                            scale: Vec3::ONE,
                        },
                        sprite: Sprite {
                            size: cell_size * cell_scale,
                            ..Default::default()
                        },
                        visible: Visible {
                            is_transparent: true,
                            is_visible: true,
                        },
                        ..Default::default()
                    })
                    .insert(BoardPart::Cell);
                //println!("x: {}, y: {}", cell_x, cell_y);
            }
        }
    }
}

fn setup(mut commands: Commands, mut spawn: EventWriter<BoardSpawnEvent>) {
    // create the board on startup, will be event driven from there
    spawn.send(BoardSpawnEvent);
}
