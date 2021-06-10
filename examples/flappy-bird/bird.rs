use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    sprite::collide_aabb::collide,
};

use crate::animation;
use crate::game_state;
use crate::gamedata;
use crate::physics;
use crate::pipes;

use animation::*;
use game_state::*;
use gamedata::*;
use physics::*;
use pipes::*;

pub struct Player;
pub struct JumpHeight(pub f32);

// data for animating rotation
pub struct VelocityRotator {
    pub angle_up: f32,
    pub angle_down: f32,
    // The amount of velocity to reach the min or max angle
    pub velocity_max: f32,
}

pub struct BirdPlugin;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum BirdMovement {
    Input,
}

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<GameOverEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(bird_setup.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(bird_input_playing.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(bird_exit.system()))
            .add_system(player_bounds_system.system().after(BirdMovement::Input))
            .add_system(player_collision_system.system().label(BirdMovement::Input))
            .add_system(velocity_rotator_system.system().label(BirdMovement::Input))
            .add_system(velocity_animator_system.system().label(BirdMovement::Input))
            .add_system(game_over.system().after(BirdMovement::Input));
    }
}



fn bird_input_playing(
    jump_height: Res<JumpHeight>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, With<Player>)>,
) {
    for (mut velocity, _) in query.iter_mut() {
        // Handle Jump
        if keyboard_input.just_pressed(KeyCode::Space) {
            velocity.0.y = jump_height.0;
        }
    }
}

struct GameOverEvent;

fn player_bounds_system(
    mut query: Query<(&Player, &mut Transform, &mut Velocity)>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    let half_screen_size = 1280.0 * 0.5;
    let player_size = 32.0 * 6.0;
    for (_p, mut transform, mut velocity) in query.iter_mut() {
        // bounce against ceiling
        if transform.translation.y > half_screen_size - player_size {
            velocity.0.y = -3.0;
            transform.translation.y = half_screen_size - player_size;
        }
        // death on bottom touch
        if transform.translation.y < -half_screen_size {
            game_over_writer.send(GameOverEvent);
        }
    }
}

fn player_collision_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    player_query: Query<(&Player, &Transform)>,
    pipe_query: Query<(&Pipe, &Transform, &Collider, &Sprite, Entity)>,
    score_collider_query: Query<(&Transform, &Collider, Entity)>,

    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    // Player size can't be fetched from AtlasTextureSprite, so I'm hard coding it here...
    let mut player_size = 6.0 * 32.0;
    // Make player hitbox half size, to feel more fair
    player_size *= 0.4;
    let player_size_vec = (player_size, player_size);
    for (_player, player_transform) in &mut player_query.iter() {
        for (score_transform, collider, entity) in &mut score_collider_query.iter() {
            if *collider != Collider::ScoreGiver {
                continue;
            }
            let collision = collide(
                player_transform.translation,
                player_size_vec.into(),
                score_transform.translation,
                Vec2::new(10.0, 1280.0),
            );
            if collision.is_some() {
                game_data.score += 1;
                // Remove coin collider, quick simple solution
                commands.entity(entity).despawn();
            }
        }
        // Check for collision
        let mut did_collide = false;
        for (_pipe, pipe_transform, _collider, pipe_sprite, _pipe_entity) in &mut pipe_query.iter()
        {
            let collision = collide(
                player_transform.translation,
                player_size_vec.into(),
                pipe_transform.translation,
                pipe_sprite.size * 6.0,
            );
            if collision.is_some() {
                did_collide = true;
                break;
            }
        }
        if did_collide {
            game_over_writer.send(GameOverEvent);
        }
    }
}

fn game_over(mut state: ResMut<State<GameState>>, mut reader: EventReader<GameOverEvent>) {
    for _ in reader.iter() {
        if *state.current() != GameState::Dead {
            state.set(GameState::Dead).unwrap();
        }
    }
}

fn velocity_rotator_system(mut query: Query<(&mut Transform, &Velocity, &VelocityRotator)>) {
    for (mut transform, velocity, velocity_rotator) in query.iter_mut() {
        //let quat = Quat::from_rotation_z(velocity_rotator.).lerp();
        let mut procentage = velocity.0.y / velocity_rotator.velocity_max;
        procentage = procentage.max(-1.0);
        procentage = procentage.min(1.0);
        // convert from -1 -> 1 to: 0 -> 1
        procentage = (procentage + 1.0) * 0.5;

        // Lerp from lower angle to upper angle
        let rad_angle = (1.0 - procentage) * velocity_rotator.angle_down
            + procentage * velocity_rotator.angle_up;

        transform.rotation = Quat::from_rotation_z(rad_angle);
    }
}

fn velocity_animator_system(mut query: Query<(&mut Animations, &Velocity)>) {
    for (mut animations, velocity) in &mut query.iter_mut() {
        if velocity.0.y > 0.0 {
            animations.current_animation = 0;
        } else {
            animations.current_animation = 1;
        }
    }
}

pub fn bird_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_bird_handle = asset_server.load("bird.png");
    let texture_atlas = TextureAtlas::from_grid(texture_bird_handle, Vec2::new(64., 64.), 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::splat(2.0),
                translation: Vec3::new(0.0, 0.0, 100.0),
                rotation: Quat::IDENTITY,
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
        .insert(Timer::from_seconds(0.1, true))
        .insert(Player)
        .insert(AffectedByGravity)
        .insert(VelocityRotator {
            angle_up: std::f32::consts::PI * 0.5 * 0.7,
            angle_down: -std::f32::consts::PI * 0.5 * 0.5,
            velocity_max: 400.0,
        })
        .insert(Velocity(Vec2::ZERO))
        .insert(Animations {
            animations: vec![
                Animation {
                    current_frame: 0,
                    frames: vec![
                        AnimationFrame {
                            index: 0,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 1,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 2,
                            time: 0.3,
                        },
                        AnimationFrame {
                            index: 1,
                            time: 0.1,
                        },
                    ],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![AnimationFrame {
                        index: 3,
                        time: 0.2,
                    }],
                },
            ],
            current_animation: 0,
        });
}

pub fn bird_exit(mut commands: Commands, mut q: Query<(Entity, With<Player>)>) {
    for (e, _) in q.iter_mut() {
        commands.entity(e).despawn();
    }
}
