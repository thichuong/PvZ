use crate::components::{GridCell, Plant, PlantType, Zombie, ZombieState};
use crate::constants::{
    COLOR_PEASHOOTER_HEAD, COLOR_PEASHOOTER_SNOUT, COLOR_PEASHOOTER_STEM, COLOR_POTATOMINE_ARMED,
    COLOR_POTATOMINE_BODY, COLOR_SUNFLOWER_FACE, COLOR_SUNFLOWER_PETALS, COLOR_SUNFLOWER_STEM,
    COLOR_WALLNUT_BODY, COLOR_WALLNUT_FACE, COLOR_ZOMBIE_ARM, COLOR_ZOMBIE_BODY, COLOR_ZOMBIE_HEAD,
    COLOR_ZOMBIE_LEGS, COLS, ROWS, SCREEN_WIDTH, TILE_SIZE,
};
use crate::resources::SpawnTimer;
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_zombies(mut commands: Commands, time: Res<Time>, mut spawn_timer: ResMut<SpawnTimer>) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished() {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..ROWS);

        // Match grid calcs from setup
        // let grid_width_px = COLS as f32 * TILE_SIZE; // Unused
        let grid_height_px = ROWS as f32 * TILE_SIZE;
        let start_x = -SCREEN_WIDTH / 2.0 + TILE_SIZE / 2.0 + 50.0;
        let start_y = -grid_height_px / 2.0 + TILE_SIZE / 2.0;

        let x = (COLS as f32).mul_add(TILE_SIZE, start_x) + 40.0; // Start off-screen right
        let y = (row as f32).mul_add(TILE_SIZE, start_y);

        let parent = commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(x, y, 2.0),
                    ..default()
                },
                Zombie {
                    health: 100.0,
                    state: ZombieState::Walking,
                },
            ))
            .id();

        commands.entity(parent).with_children(|parent| {
            // Legs
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_ZOMBIE_LEGS,
                    custom_size: Some(Vec2::new(10.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-10.0, -30.0, 0.1),
                ..default()
            });
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_ZOMBIE_LEGS,
                    custom_size: Some(Vec2::new(10.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(10.0, -30.0, 0.1),
                ..default()
            });

            // Body
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_ZOMBIE_BODY,
                    custom_size: Some(Vec2::new(30.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.2),
                ..default()
            });

            // Head
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_ZOMBIE_HEAD,
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 40.0, 0.3),
                ..default()
            });

            // Arm
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_ZOMBIE_ARM,
                    custom_size: Some(Vec2::new(40.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-20.0, 0.0, 0.3),
                ..default()
            });
        });
    }
}

pub fn spawn_plant(
    commands: &mut Commands,
    plant_type: PlantType,
    x: f32,
    y: f32,
    col: i32,
    row: i32,
) {
    // Default logic
    let mut health = 100.0;
    let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    let armed = false;

    match plant_type {
        PlantType::Peashooter => timer = Timer::from_seconds(1.5, TimerMode::Repeating),
        PlantType::Sunflower => timer = Timer::from_seconds(5.0, TimerMode::Repeating),
        PlantType::WallNut => health = 1000.0,
        PlantType::PotatoMine => {
            timer = Timer::from_seconds(2.0, TimerMode::Once); // Arming time
            health = 50.0;
        }
    }

    let parent = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(x, y, 1.0),
                ..default()
            },
            Plant {
                kind: plant_type,
                timer,
                health,
                armed,
            },
            GridCell { x: col, y: row },
        ))
        .id();

    commands.entity(parent).with_children(|parent| {
        spawn_plant_visuals(parent, plant_type, 1.0);
    });
}

pub fn spawn_plant_visuals(parent: &mut ChildBuilder, plant_type: PlantType, alpha: f32) {
    match plant_type {
        PlantType::Peashooter => {
            // Stem
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PEASHOOTER_STEM.with_a(alpha),
                    custom_size: Some(Vec2::new(10.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, -20.0, 0.1),
                ..default()
            });
            // Head
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PEASHOOTER_HEAD.with_a(alpha),
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 15.0, 0.2),
                ..default()
            });
            // Snout
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PEASHOOTER_SNOUT.with_a(alpha),
                    custom_size: Some(Vec2::new(20.0, 15.0)),
                    ..default()
                },
                transform: Transform::from_xyz(20.0, 15.0, 0.1),
                ..default()
            });
        }
        PlantType::Sunflower => {
            // Stem
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_SUNFLOWER_STEM.with_a(alpha),
                    custom_size: Some(Vec2::new(10.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, -20.0, 0.1),
                ..default()
            });
            // Petals
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_SUNFLOWER_PETALS.with_a(alpha),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 10.0, 0.2),
                ..default()
            });
            // Face
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_SUNFLOWER_FACE.with_a(alpha),
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 10.0, 0.3),
                ..default()
            });
        }
        PlantType::WallNut => {
            // Body
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_WALLNUT_BODY.with_a(alpha),
                    custom_size: Some(Vec2::new(50.0, 60.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.2),
                ..default()
            });
            // Eyes
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_WALLNUT_FACE.with_a(alpha),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-10.0, 10.0, 0.3),
                ..default()
            });
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_WALLNUT_FACE.with_a(alpha),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_xyz(10.0, 10.0, 0.3),
                ..default()
            });
        }
        PlantType::PotatoMine => {
            // Determine if armed color (will act as visual indicator)
            // Actually, visual update is better in system, but for now just spawn static
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_POTATOMINE_BODY.with_a(alpha),
                    custom_size: Some(Vec2::new(30.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, -20.0, 0.2),
                ..default()
            });
            // Red light (initially off/small)
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_POTATOMINE_ARMED.with_a(alpha),
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, -10.0, 0.3),
                ..default()
            });
        }
    }
}
