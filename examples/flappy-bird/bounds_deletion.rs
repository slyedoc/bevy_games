use bevy::prelude::*;

pub struct OffsceenDeletion;

pub struct BoundsDeletionPlugin;

impl Plugin for BoundsDeletionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(offscreen_remove_system.system());
    }
}

fn offscreen_remove_system(
    mut commands: Commands,
    mut pipe_query: Query<(Entity, &mut Transform, &OffsceenDeletion)>,
) {
    let padding = 300.0;
    for (entity, trans, _od) in &mut pipe_query.iter_mut() {
        // Left side of screen
        if trans.translation.x < -1920.0 * 0.5 - padding {
            commands.entity(entity).despawn();
        }
    }
}
