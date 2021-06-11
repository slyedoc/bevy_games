use crate::game_state;
use crate::physics;
use bevy::{prelude::*, utils::Duration};
use game_state::*;
use physics::*;
use rand::{thread_rng, Rng};

pub struct BoardMove(Timer);

pub struct BoardMovePlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(BoardMove(Timer::from_seconds(1.0, true)))
            .add_system(move_setup.system()),
            );
    }
}

fn move_setup(
    mut commands: Commands,
    time: Res<Time>,
    mut move_timer: ResMut<BoardMove>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {


    cloud_timer
        .0
        .tick(Duration::from_secs_f32(time.delta_seconds()));
    if cloud_timer.0.finished() {
        let scale = rng.gen_range(6.0..30.0);
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(cloud_texture.into()),
                transform: Transform {
                    scale: Vec3::new(scale, scale, scale),
                    translation: Vec3::new(
                        1920.0 * 0.5 + 30.0 * 43.0,
                        rng.gen_range(-1280.0 * 0.5..1280.0 * 0.5),
                        2.0,
                    ),
                    rotation: Quat::IDENTITY,
                },
                ..Default::default()
            })
            .insert(Velocity(Vec2::new(
                rng.gen_range(-700.0..-400.0),
                rng.gen_range(-10.0..10.0),
            )));
    }
}

