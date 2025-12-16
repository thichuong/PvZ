use crate::components::{Plant, PlantType, Bullet, Zombie, ZombieState};
use crate::constants::{COLOR_BULLET, BULLET_SPEED, SCREEN_WIDTH, ZOMBIE_SPEED};
use crate::resources::GameState;
use bevy::prelude::*;

pub fn plant_action(
    mut commands: Commands,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut query: Query<(&mut Plant, &Transform)>,
) {
    for (mut plant, transform) in &mut query {
        plant.timer.tick(time.delta());

        match plant.plant_type {
            PlantType::Peashooter => {
                if plant.timer.finished() {
                    let spawn_pos = transform.translation + Vec3::new(40.0, 20.0, 3.0);
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: COLOR_BULLET,
                                custom_size: Some(Vec2::new(15.0, 15.0)),
                                ..default()
                            },
                            transform: Transform::from_translation(spawn_pos),
                            ..default()
                        },
                        Bullet,
                    ));
                }
            }
            PlantType::Sunflower => {
                if plant.timer.finished() {
                    game_state.sun += 25;
                }
            }
            PlantType::PotatoMine => {
                if plant.timer.finished() && !plant.armed {
                    plant.armed = true;
                    // Visual indication? For prototype, maybe just logic.
                }
            }
            _ => {}
        }
    }
}

pub fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Bullet>>,
) {
    for (entity, mut transform) in &mut query {
        transform.translation.x += BULLET_SPEED * time.delta_seconds();
        if transform.translation.x > SCREEN_WIDTH / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn move_zombies(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Zombie)>,
) {
    for (entity, mut transform, zombie) in &mut query {
        if zombie.state == ZombieState::Walking {
            transform.translation.x -= ZOMBIE_SPEED * time.delta_seconds();
        }

        if transform.translation.x < -SCREEN_WIDTH / 2.0 - 50.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
