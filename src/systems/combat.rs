use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;

// Logic for zombies eating plants + collisions
pub fn zombie_eat_system(
    mut commands: Commands,
    time: Res<Time>,
    mut zombie_query: Query<(Entity, &Transform, &mut Zombie)>,
    mut plant_query: Query<(Entity, &Transform, &mut Plant)>,
) {
    for (zombie_entity, zombie_transform, mut zombie) in zombie_query.iter_mut() {
        match zombie.state {
            ZombieState::Walking => {
                // Check if colliding with any plant
                for (plant_entity, plant_transform, plant) in plant_query.iter() {
                    let distance = zombie_transform
                        .translation
                        .truncate()
                        .distance(plant_transform.translation.truncate());
                    if distance < 40.0 {
                        // Collision!

                        // Special Case: Potato Mine
                        if plant.plant_type == PlantType::PotatoMine && plant.armed {
                            // BOOM
                            commands.entity(plant_entity).despawn_recursive();
                            commands.entity(zombie_entity).despawn_recursive();
                            break;
                        }

                        // Normal: Start eating
                        zombie.state = ZombieState::Eating(plant_entity);
                        break; // Only eat one at a time
                    }
                }
            }
            ZombieState::Eating(plant_entity) => {
                // Check if plant still exists and damage it
                if let Ok((_, _, mut plant)) = plant_query.get_mut(plant_entity) {
                    plant.health -= ZOMBIE_EAT_DPS * time.delta_seconds();
                    if plant.health <= 0.0 {
                        commands.entity(plant_entity).despawn_recursive();
                        zombie.state = ZombieState::Walking;
                    }
                } else {
                    // Plant gone, resume walking
                    zombie.state = ZombieState::Walking;
                }
            }
        }
    }
}

pub fn collision_system(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut zombie_query: Query<(Entity, &Transform, &mut Zombie)>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (zombie_entity, zombie_transform, mut zombie) in zombie_query.iter_mut() {
            let distance = bullet_transform
                .translation
                .truncate()
                .distance(zombie_transform.translation.truncate());
            if distance < 40.0 {
                // hit radius
                commands.entity(bullet_entity).despawn();
                zombie.health -= 10.0;
                if zombie.health <= 0.0 {
                    commands.entity(zombie_entity).despawn_recursive();
                }
            }
        }
    }
}
