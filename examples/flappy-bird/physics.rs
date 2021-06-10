use crate::game_state;
use bevy::prelude::*;
use game_state::*;

pub struct Velocity(pub Vec2);

pub struct Gravity(pub f32);
pub struct AffectedByGravity;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(velocity_system.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(gravity_system.system()),
        );
    }
}

fn gravity_system(
    gravity: Res<Gravity>,
    time: Res<Time>,
    mut q: Query<(With<AffectedByGravity>, &mut Velocity)>,
) {
    for (_, mut vel) in q.iter_mut() {
        vel.0.y -= gravity.0 * time.delta_seconds();
    }
}

fn velocity_system(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut trans) in q.iter_mut() {
        let x = trans.translation.x;
        let y = trans.translation.y;
        let delta = time.delta_seconds();
        trans.translation = Vec3::new(x + vel.0.x * delta, y + vel.0.y * delta, 0.);
    }
}
