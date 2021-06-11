use crate::utils;
use crate::game_state;
use bevy::{
    prelude::*,
    math::*,
};
use utils::*;
use game_state::*;


pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .insert_resource(BoardSize{ x: 19, y: 19 })
        .add_startup_system(setup.system());
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct BoardSize {
    x: u8,
    y: u8
}

enum BoardPart {
    Background,
    Cell{ x: u8, y: u8 },
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    board_size: Res<BoardSize>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

        let size: f32 = 500.;
        let cell_size = Vec2::new( -size / board_size.x as f32 / 2., size / board_size.x as f32 / 2.);
        let background_size = Vec2::new( -size + cell_size.x * 2., size + cell_size.y * 2.);
        let board_trim: f32 = 1.1;

        // board-trim
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::BLACK.into()),
            transform: Transform {
                translation:  Vec3::new(0., 0., 0.),
                rotation:  Quat::IDENTITY,
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
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::SILVER.into()),
            transform: Transform {
                translation:  Vec3::new(0., 0., 0.),
                rotation:  Quat::IDENTITY,
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
        let thinkness = 5.;
        for x in 0..board_size.x {
            let x = utils::map_range(x as f32, 0.0, (board_size.x - 1) as f32, -size / 2., size / 2.);
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::BLACK.into()),
                transform: Transform {
                    translation:  Vec3::new(x, 0., 0.),
                    rotation:  Quat::IDENTITY,
                    scale: Vec3::ONE,
                },
                sprite: Sprite {
                    size: Vec2::new(thinkness, size ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(BoardPart::Background);
        }

        for y in 0..board_size.y {
            let y = utils::map_range(y as f32, 0.0, (board_size.y - 1) as f32, -size / 2., size / 2.);
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::BLACK.into()),
                transform: Transform {
                    translation:  Vec3::new(0., y, 0.),
                    rotation:  Quat::IDENTITY,
                    scale: Vec3::ONE,
                },
                sprite: Sprite {
                    size: Vec2::new(size, thinkness ),
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
                let x = utils::map_range(cell_x as f32, 0.0, (board_size.x - 1) as f32, -size / 2., size / 2.);
                let y = utils::map_range(cell_y as f32, 0.0, (board_size.y - 1) as f32, -size / 2., size / 2.);

                // Create board peice
                commands.spawn_bundle(SpriteBundle {
                    material: materials.add(Color::RED.into()),
                    transform: Transform {
                        translation:  Vec3::new(x,  y , 0.),
                        rotation:  Quat::IDENTITY,
                        scale: Vec3::ONE,
                    },
                    sprite: Sprite {
                        size: cell_size * cell_scale,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(BoardPart::Cell { x: cell_x, y: cell_y });
                //println!("x: {}, y: {}", cell_x, cell_y);

            }
        }
    }

