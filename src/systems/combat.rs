use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;
use rand::Rng;

// Logic for zombies eating plants + collisions
pub fn zombie_eat_system(
    mut commands: Commands,
    time: Res<Time>,
    mut zombie_query: Query<(Entity, &Transform, &mut Zombie)>,
    mut plant_query: Query<(Entity, &Transform, &mut Plant)>,
) {
    for (_zombie_entity, zombie_transform, mut zombie) in zombie_query.iter_mut() {
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
                            // BOOM - Spawn explosion
                            commands.spawn((
                                Explosion {
                                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                                    radius: 120.0, // 3x3 approx (1.5 * 80)
                                    damage: 1000.0,
                                },
                                SpatialBundle {
                                    transform: *plant_transform,
                                    ..default()
                                },
                            ));
                            commands.entity(plant_entity).despawn_recursive();

                            // Spawn Particles
                            let mut rng = rand::thread_rng();
                            for _ in 0..12 {
                                let vx: f32 = rng.gen_range(-150.0..150.0);
                                let vy: f32 = rng.gen_range(-150.0..150.0);
                                commands.spawn((
                                    SpriteBundle {
                                        sprite: Sprite {
                                            color: Color::rgb(0.8, 0.5, 0.2), // Potato color ish
                                            custom_size: Some(Vec2::new(10.0, 10.0)),
                                            ..default()
                                        },
                                        transform: *plant_transform,
                                        ..default()
                                    },
                                    Particle {
                                        velocity: Vec2::new(vx, vy),
                                        timer: Timer::from_seconds(0.8, TimerMode::Once),
                                    },
                                ));
                            }

                            // Zombie is not despawned here, will be caught by explosion system
                            break;
                        }

                        // Normal: Start eating
                        zombie.state = ZombieState::Eating(plant_entity);
                        break; // Only eat one at a time
                    }
                }
            }
            ZombieState::Eating(plant_entity) => {
                // Check if plant still exists
                if let Ok((p_entity, p_transform, mut plant)) = plant_query.get_mut(plant_entity) {
                    // CHECK: If it turned into an ACTVE potato mine while being eaten, BOOM
                    if plant.plant_type == PlantType::PotatoMine && plant.armed {
                        commands.spawn((
                            Explosion {
                                timer: Timer::from_seconds(0.5, TimerMode::Once),
                                radius: 120.0,
                                damage: 1000.0,
                            },
                            SpatialBundle {
                                transform: *p_transform,
                                ..default()
                            },
                        ));
                        commands.entity(p_entity).despawn_recursive();

                        // Spawn Particles
                        let mut rng = rand::thread_rng();
                        for _ in 0..12 {
                            let vx: f32 = rng.gen_range(-150.0..150.0);
                            let vy: f32 = rng.gen_range(-150.0..150.0);
                            commands.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::rgb(0.8, 0.5, 0.2),
                                        custom_size: Some(Vec2::new(10.0, 10.0)),
                                        ..default()
                                    },
                                    transform: *p_transform,
                                    ..default()
                                },
                                Particle {
                                    velocity: Vec2::new(vx, vy),
                                    timer: Timer::from_seconds(0.8, TimerMode::Once),
                                },
                            ));
                        }

                        zombie.state = ZombieState::Walking; // Stop eating
                        continue;
                    }

                    plant.health -= ZOMBIE_EAT_DPS * time.delta_seconds();
                    if plant.health <= 0.0 {
                        commands.entity(p_entity).despawn_recursive();
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
// Explosion logic
pub fn explosion_damage_system(
    mut commands: Commands,
    time: Res<Time>,
    mut explosion_query: Query<(Entity, &Transform, &mut Explosion)>,
    mut zombie_query: Query<(Entity, &Transform, &mut Zombie)>,
) {
    for (exp_entity, exp_transform, mut explosion) in explosion_query.iter_mut() {
        // Apply damage to all zombies in range
        let exp_pos = exp_transform.translation.truncate();

        for (zombie_entity, zombie_transform, mut zombie) in zombie_query.iter_mut() {
            let z_pos = zombie_transform.translation.truncate();
            if exp_pos.distance(z_pos) <= explosion.radius {
                zombie.health -= explosion.damage * time.delta_seconds();
                if zombie.health <= 0.0 {
                    commands.entity(zombie_entity).despawn_recursive();
                }
            }
        }

        explosion.timer.tick(time.delta());
        if explosion.timer.finished() {
            commands.entity(exp_entity).despawn_recursive();
        }
    }
}
