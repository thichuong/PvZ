use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::Rng;

// --- Constants ---
const TILE_SIZE: f32 = 80.0;
const ROWS: i32 = 5;
const COLS: i32 = 9;
const SCREEN_WIDTH: f32 = COLS as f32 * TILE_SIZE + 200.0; // Extra width for HUD
const SCREEN_HEIGHT: f32 = ROWS as f32 * TILE_SIZE + 100.0;

// Colors
const COLOR_GRASS_1: Color = Color::rgb(0.0, 0.4, 0.0);
const COLOR_GRASS_2: Color = Color::rgb(0.0, 0.35, 0.0);
const COLOR_SUN_TEXT: Color = Color::WHITE;
const COLOR_SELECTED_TEXT: Color = Color::GOLD;

// Plant Colors
const COLOR_PEASHOOTER_STEM: Color = Color::rgb(0.0, 0.3, 0.0);
const COLOR_PEASHOOTER_HEAD: Color = Color::rgb(0.2, 0.8, 0.2);
const COLOR_PEASHOOTER_SNOUT: Color = Color::rgb(0.1, 0.6, 0.1);

const COLOR_SUNFLOWER_STEM: Color = Color::rgb(0.0, 0.3, 0.0);
const COLOR_SUNFLOWER_PETALS: Color = Color::rgb(1.0, 1.0, 0.0);
const COLOR_SUNFLOWER_FACE: Color = Color::rgb(0.4, 0.2, 0.0);

const COLOR_WALLNUT_BODY: Color = Color::rgb(0.6, 0.4, 0.2);
const COLOR_WALLNUT_FACE: Color = Color::BLACK;

const COLOR_POTATOMINE_BODY: Color = Color::rgb(0.5, 0.4, 0.3);
const COLOR_POTATOMINE_ARMED: Color = Color::RED;

// Zombie Colors
const COLOR_ZOMBIE_LEGS: Color = Color::rgb(0.2, 0.2, 0.2);
const COLOR_ZOMBIE_BODY: Color = Color::rgb(0.2, 0.2, 0.6);
const COLOR_ZOMBIE_HEAD: Color = Color::rgb(0.6, 0.7, 0.6);
const COLOR_ZOMBIE_ARM: Color = Color::rgb(0.2, 0.2, 0.6);

const COLOR_BULLET: Color = Color::rgb(0.0, 1.0, 1.0);

const ZOMBIE_SPEED: f32 = 20.0;
const BULLET_SPEED: f32 = 200.0;
const ZOMBIE_EAT_DPS: f32 = 20.0; // Damage per second when eating

// Costs
const COST_PEASHOOTER: u32 = 100;
const COST_SUNFLOWER: u32 = 50;
const COST_WALLNUT: u32 = 50;
const COST_POTATOMINE: u32 = 25;

// --- Components ---

#[derive(Component)]
struct Plant {
    plant_type: PlantType,
    timer: Timer,
    health: f32,

    // Potato Mine specific
    armed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlantType {
    Peashooter,
    Sunflower,
    WallNut,
    PotatoMine,
}

#[derive(Component)]
struct Zombie {
    health: f32,
    state: ZombieState,
}

#[derive(PartialEq)]
enum ZombieState {
    Walking,
    Eating(Entity), // Entity being eaten
}

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct GridCell {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct SunText;

// Marked for buttons
#[derive(Component)]
struct PlantButton(PlantType);

// --- Resources ---

#[derive(Resource)]
struct GameState {
    sun: u32,
    selected_plant: PlantType,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            sun: 150,
            selected_plant: PlantType::Peashooter,
        }
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

// --- Main ---

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                title: "PvZ Bevy Prototype".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameState::default())
        .insert_resource(SpawnTimer(Timer::from_seconds(10.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                input_system,
                button_system, // UI Clicks
                spawn_zombies,
                plant_action,
                move_bullets,
                move_zombies,
                zombie_eat_system, // Interactions
                collision_system,
                ui_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Grid Background (Left side)
    let grid_width_px = COLS as f32 * TILE_SIZE;
    let grid_height_px = ROWS as f32 * TILE_SIZE;
    let start_x = -SCREEN_WIDTH / 2.0 + TILE_SIZE / 2.0 + 50.0; // Left align with margin
    let start_y = -grid_height_px / 2.0 + TILE_SIZE / 2.0;

    for row in 0..ROWS {
        for col in 0..COLS {
            let color = if (row + col) % 2 == 0 {
                COLOR_GRASS_1
            } else {
                COLOR_GRASS_2
            };

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    start_x + col as f32 * TILE_SIZE,
                    start_y + row as f32 * TILE_SIZE,
                    0.0,
                ),
                ..default()
            });
        }
    }

    // UI Panel (Right side)
    let panel_x = SCREEN_WIDTH / 2.0 - 90.0;

    // Sun Text
    commands.spawn((
        TextBundle::from_section(
            "Sun: 150",
            TextStyle {
                font_size: 30.0,
                color: COLOR_SUN_TEXT,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        SunText,
    ));

    // Plant Buttons
    let plants = [
        (
            PlantType::Peashooter,
            "Pea",
            COST_PEASHOOTER,
            COLOR_PEASHOOTER_HEAD,
        ),
        (
            PlantType::Sunflower,
            "Sun",
            COST_SUNFLOWER,
            COLOR_SUNFLOWER_PETALS,
        ),
        (PlantType::WallNut, "Nut", COST_WALLNUT, COLOR_WALLNUT_BODY),
        (
            PlantType::PotatoMine,
            "Mine",
            COST_POTATOMINE,
            COLOR_POTATOMINE_BODY,
        ),
    ];

    for (i, (ptype, label, cost, color)) in plants.iter().enumerate() {
        commands
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(80.0),
                    height: Val::Px(60.0),
                    position_type: PositionType::Absolute,
                    right: Val::Px(10.0),
                    top: Val::Px(50.0 + i as f32 * 70.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            })
            .insert(PlantButton(*ptype))
            .with_children(|parent| {
                // Icon preview
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: (*color).into(),
                    ..default()
                });
                parent.spawn(
                    TextBundle::from_section(
                        format!("{}\n{}", label, cost),
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )
                    .with_text_justify(JustifyText::Center),
                );
            });
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &PlantButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, button, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                game_state.selected_plant = button.0;
                *color = Color::GRAY.into();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                if game_state.selected_plant == button.0 {
                    *color = Color::rgb(0.3, 0.3, 0.3).into(); // Highlight selected
                } else {
                    *color = Color::DARK_GRAY.into();
                }
            }
        }
    }
}

fn input_system(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut game_state: ResMut<GameState>,
    existing_plants: Query<&GridCell, With<Plant>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Grid geometry
            let grid_width_px = COLS as f32 * TILE_SIZE;
            let grid_height_px = ROWS as f32 * TILE_SIZE;
            let start_x = -SCREEN_WIDTH / 2.0 + TILE_SIZE / 2.0 + 50.0;
            let start_y = -grid_height_px / 2.0 + TILE_SIZE / 2.0;

            // Manual inverse mapping due to start_x offset
            let grid_base_x = start_x - TILE_SIZE / 2.0;
            let grid_base_y = start_y - TILE_SIZE / 2.0;

            let col = ((world_position.x - grid_base_x) / TILE_SIZE).floor() as i32;
            let row = ((world_position.y - grid_base_y) / TILE_SIZE).floor() as i32;

            if col >= 0 && col < COLS && row >= 0 && row < ROWS {
                // Check occupancy
                if existing_plants
                    .iter()
                    .any(|cell| cell.x == col && cell.y == row)
                {
                    return;
                }

                // Check cost
                let cost = match game_state.selected_plant {
                    PlantType::Peashooter => COST_PEASHOOTER,
                    PlantType::Sunflower => COST_SUNFLOWER,
                    PlantType::WallNut => COST_WALLNUT,
                    PlantType::PotatoMine => COST_POTATOMINE,
                };

                if game_state.sun >= cost {
                    game_state.sun -= cost;
                    // Center of cell
                    let pos_x = start_x + col as f32 * TILE_SIZE;
                    let pos_y = start_y + row as f32 * TILE_SIZE;
                    spawn_plant(
                        &mut commands,
                        game_state.selected_plant,
                        pos_x,
                        pos_y,
                        col,
                        row,
                    );
                }
            }
        }
    }
}

fn spawn_plant(commands: &mut Commands, plant_type: PlantType, x: f32, y: f32, col: i32, row: i32) {
    // Default logic
    let mut health = 100.0;
    let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    let mut armed = false;

    match plant_type {
        PlantType::Peashooter => timer = Timer::from_seconds(1.5, TimerMode::Repeating),
        PlantType::Sunflower => timer = Timer::from_seconds(5.0, TimerMode::Repeating),
        PlantType::WallNut => health = 1000.0,
        PlantType::PotatoMine => {
            timer = Timer::from_seconds(15.0, TimerMode::Once); // Arming time
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
                plant_type,
                timer,
                health,
                armed,
            },
            GridCell { x: col, y: row },
        ))
        .id();

    match plant_type {
        PlantType::Peashooter => {
            commands.entity(parent).with_children(|parent| {
                // Stem
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_PEASHOOTER_STEM,
                        custom_size: Some(Vec2::new(10.0, 40.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -20.0, 0.1),
                    ..default()
                });
                // Head
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_PEASHOOTER_HEAD,
                        custom_size: Some(Vec2::new(30.0, 30.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 15.0, 0.2),
                    ..default()
                });
                // Snout
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_PEASHOOTER_SNOUT,
                        custom_size: Some(Vec2::new(20.0, 15.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(20.0, 15.0, 0.1),
                    ..default()
                });
            });
        }
        PlantType::Sunflower => {
            commands.entity(parent).with_children(|parent| {
                // Stem
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_SUNFLOWER_STEM,
                        custom_size: Some(Vec2::new(10.0, 40.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -20.0, 0.1),
                    ..default()
                });
                // Petals
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_SUNFLOWER_PETALS,
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 10.0, 0.2),
                    ..default()
                });
                // Face
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_SUNFLOWER_FACE,
                        custom_size: Some(Vec2::new(30.0, 30.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 10.0, 0.3),
                    ..default()
                });
            });
        }
        PlantType::WallNut => {
            commands.entity(parent).with_children(|parent| {
                // Body
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_WALLNUT_BODY,
                        custom_size: Some(Vec2::new(50.0, 60.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.2),
                    ..default()
                });
                // Eyes
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_WALLNUT_FACE,
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-10.0, 10.0, 0.3),
                    ..default()
                });
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_WALLNUT_FACE,
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(10.0, 10.0, 0.3),
                    ..default()
                });
            });
        }
        PlantType::PotatoMine => {
            commands.entity(parent).with_children(|parent| {
                // Determine if armed color (will act as visual indicator)
                // Actually, visual update is better in system, but for now just spawn static
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_POTATOMINE_BODY,
                        custom_size: Some(Vec2::new(30.0, 20.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -20.0, 0.2),
                    ..default()
                });
                // Red light (initially off/small)
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_POTATOMINE_ARMED,
                        custom_size: Some(Vec2::new(5.0, 5.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -10.0, 0.3),
                    ..default()
                });
            });
        }
    }
}

fn spawn_zombies(mut commands: Commands, time: Res<Time>, mut spawn_timer: ResMut<SpawnTimer>) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished() {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..ROWS);

        // Match grid calcs from setup
        let grid_width_px = COLS as f32 * TILE_SIZE;
        let grid_height_px = ROWS as f32 * TILE_SIZE;
        let start_x = -SCREEN_WIDTH / 2.0 + TILE_SIZE / 2.0 + 50.0;
        let start_y = -grid_height_px / 2.0 + TILE_SIZE / 2.0;

        let x = start_x + (COLS as f32) * TILE_SIZE + 40.0; // Start off-screen right
        let y = start_y + row as f32 * TILE_SIZE;

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

fn plant_action(
    mut commands: Commands,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut query: Query<(&mut Plant, &Transform)>,
) {
    for (mut plant, transform) in query.iter_mut() {
        plant.timer.tick(time.delta());

        match plant.plant_type {
            PlantType::Peashooter => {
                if plant.timer.finished() {
                    let spawn_pos = transform.translation + Vec3::new(40.0, 20.0, 0.0);
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

fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Bullet>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x += BULLET_SPEED * time.delta_seconds();
        if transform.translation.x > SCREEN_WIDTH / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn move_zombies(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Zombie)>,
) {
    for (entity, mut transform, zombie) in query.iter_mut() {
        if zombie.state == ZombieState::Walking {
            transform.translation.x -= ZOMBIE_SPEED * time.delta_seconds();
        }

        if transform.translation.x < -SCREEN_WIDTH / 2.0 - 50.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Logic for zombies eating plants + collisions
fn zombie_eat_system(
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

fn collision_system(
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

fn ui_system(game_state: Res<GameState>, mut sun_query: Query<&mut Text, (With<SunText>)>) {
    for mut text in sun_query.iter_mut() {
        text.sections[0].value = format!("Sun: {}", game_state.sun);
    }
}
