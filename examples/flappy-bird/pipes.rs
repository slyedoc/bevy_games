use crate::bounds_deletion;
use crate::game_state;
use crate::physics;
use bevy::{prelude::*, utils::Duration};
use bounds_deletion::*;
use game_state::*;
use physics::*;
use rand::{thread_rng, Rng};

pub struct Pipe;

pub struct ScoreCollider;

pub struct SpawnTimer {
    pub timer: Timer,
    // center pos of pipes, in precentage
    pub last_pos: f32,
}

pub struct PipeSpawnSettings {
    pub min_time: f32,
    pub max_time: f32,
    pub speed: f32,
    // distance from upper and lower pipe, in precentage
    pub min_pipe_distance: f32,
    pub max_pipe_distance: f32,
    pub max_center_delta: f32,
}

#[derive(PartialEq)]
pub enum Collider {
    Solid,
    ScoreGiver,
}

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(pipe_system_update.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(pipe_system_exit.system()),
        )
        .insert_resource(SpawnTimer {
            timer: Timer::from_seconds(2.0, true),
            last_pos: 0.5,
        })
        .insert_resource(PipeSpawnSettings {
            min_time: 0.9,
            max_time: 1.2,
            speed: -700.0,
            min_pipe_distance: 300.0,
            max_pipe_distance: 600.0,
            max_center_delta: 0.4,
        });
    }
}

fn pipe_system_exit(
    mut commands: Commands,
    mut q: Query<(Entity, With<Pipe>)>,
    mut scores: Query<(Entity, With<ScoreCollider>)>,
) {
    for (e, _) in q.iter_mut() {
        commands.entity(e).despawn();
    }

    for (e, _) in scores.iter_mut() {
        commands.entity(e).despawn();
    }
}

fn pipe_system_update(
    mut commands: Commands,
    pipe_settings: Res<PipeSpawnSettings>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if spawn_timer.timer.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();
        let time = rng.gen_range(pipe_settings.min_time..pipe_settings.max_time);
        spawn_timer
            .timer
            .set_duration(Duration::from_secs_f32(time));

        let mut new_center_pos = spawn_timer.last_pos
            - rng.gen_range(-pipe_settings.max_center_delta..pipe_settings.max_center_delta);

        // sorry for the hardcoded values
        // This is the extent from the center in Y, a pipe can go maximum, until it flies in the air
        let clamp_range = (1280.0 - (6.0 * 128.0)) / 1280.0;

        // Clamp func seem to be nightly only for now
        new_center_pos = new_center_pos.min(clamp_range);
        new_center_pos = new_center_pos.max(-clamp_range);
        spawn_timer.last_pos = new_center_pos;
        // to world units
        new_center_pos *= 1280.0 * 0.5;

        let pipe_texture_handle = asset_server.load("pipe.png");
        let pipe_texture_handle2 = asset_server.load("pipe.png");

        let pipe_offset_y = (6.0 * 128.0) * 0.5;
        let pipe_offset_x = (6.0 * 32.0) * 0.5;
        let mut pipe_delta =
            rng.gen_range(pipe_settings.min_pipe_distance..pipe_settings.max_pipe_distance);

        // half the size because both pipes will be offseted in opposide direction
        pipe_delta *= 0.5;
        let x_pos = 1920.0 * 0.5 + pipe_offset_x;

        // lower pipe
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(pipe_texture_handle.into()),
                transform: Transform {
                    translation: Vec3::new(
                        x_pos,
                        -pipe_offset_y + new_center_pos - pipe_delta,
                        3.0,
                    ),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::splat(6.0),
                },
                draw: Draw {
                    render_commands: Vec::new(),
                },
                ..Default::default()
            })
            .insert(Velocity(Vec2::new(pipe_settings.speed, 0.0)))
            .insert(Pipe)
            .insert(OffsceenDeletion)
            .insert(Collider::Solid);

        // higher pipe
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(pipe_texture_handle2.into()),
                transform: Transform {
                    translation: Vec3::new(x_pos, pipe_offset_y + new_center_pos + pipe_delta, 3.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::PI),
                    scale: Vec3::splat(6.0),
                },
                draw: Draw {
                    render_commands: Vec::new(),
                },
                visible: Visible {
                    is_transparent: true,
                    is_visible: true,
                },
                ..Default::default()
            })
            .insert(Pipe)
            .insert(OffsceenDeletion)
            .insert(Velocity(Vec2::new(pipe_settings.speed, 0.0)))
            .insert(Collider::Solid);

        // score collider offseted by half player size
        let score_offset = Vec3::new(32.0 * 6.0 * 0.5, 0.0, 0.0);
        commands
            .spawn()
            .insert(Transform::from_translation(
                score_offset + Vec3::new(x_pos, 0.0, 0.0),
            ))
            .insert(ScoreCollider)
            .insert(Collider::ScoreGiver)
            .insert(Velocity(Vec2::new(pipe_settings.speed, 0.0)))
            .insert(OffsceenDeletion);
    }
}
