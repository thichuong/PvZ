use crate::components::Particle;
use bevy::prelude::*;

pub fn particle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Particle)>,
) {
    for (entity, mut transform, mut particle) in query.iter_mut() {
        // Move particle
        transform.translation += particle.velocity.extend(0.0) * time.delta_seconds();

        // Tick timer
        particle.timer.tick(time.delta());
        if particle.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
