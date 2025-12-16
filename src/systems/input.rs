use crate::components::{Cursor, GridCell, Plant, PlantType, Tool, ToolButton};
use crate::constants::{
    COLS, COST_PEASHOOTER, COST_POTATOMINE, COST_SUNFLOWER, COST_WALLNUT, ROWS, SCREEN_WIDTH,
    TILE_SIZE,
};
use crate::resources::GameState;
use crate::systems::spawning::{spawn_plant, spawn_plant_visuals};
use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &ToolButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                game_state.selected_tool = button.0;
                *color = Color::GRAY.into();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                if game_state.selected_tool == button.0 {
                    *color = Color::rgb(0.3, 0.3, 0.3).into(); // Highlight selected
                } else {
                    *color = Color::DARK_GRAY.into();
                }
            }
        }
    }
}

pub fn cursor_system(
    mut commands: Commands,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut cursor_q: Query<(Entity, &mut Transform), With<Cursor>>,
    game_state: Res<GameState>,
    mut current_tool: Local<Option<Tool>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return;
    };

    let cursor_entity = if let Ok((entity, ..)) = cursor_q.get_single() {
        entity
    } else {
        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_scale(Vec3::splat(1.0)),
                    visibility: Visibility::Visible,
                    ..default()
                },
                Cursor,
            ))
            .id()
    };

    // Update position
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if let Ok((_, mut transform)) = cursor_q.get_mut(cursor_entity) {
            transform.translation = world_position.extend(999.0);
        }
    }

    if *current_tool != Some(game_state.selected_tool) {
        *current_tool = Some(game_state.selected_tool);

        // Remove children
        commands.entity(cursor_entity).despawn_descendants();

        match game_state.selected_tool {
            Tool::Plant(pt) => {
                commands.entity(cursor_entity).with_children(|parent| {
                    spawn_plant_visuals(parent, pt, 0.5);
                });
            }
            Tool::Shovel => {
                commands.entity(cursor_entity).with_children(|parent| {
                    // Simple Shovel visual (copying setup)
                    // Handle Loop
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.4, 0.2, 0.1).with_a(0.8), // Brown transparent
                            custom_size: Some(Vec2::new(12.0, 4.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 15.0, 0.1),
                        ..default()
                    });
                    // Shaft
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.4, 0.2, 0.1).with_a(0.8),
                            custom_size: Some(Vec2::new(4.0, 12.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 7.0, 0.1),
                        ..default()
                    });
                    // Blade
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::SILVER.with_a(0.8),
                            custom_size: Some(Vec2::new(16.0, 14.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, -5.0, 0.1),
                        ..default()
                    });
                });
            }
        }
    }
}

pub fn input_system(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut game_state: ResMut<GameState>,
    existing_plants: Query<(Entity, &GridCell), With<Plant>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Ok((camera, camera_transform)) = camera_q.get_single() else {
            return;
        };
        let Ok(window) = windows.get_single() else {
            return;
        };

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Grid geometry
            // let grid_width_px = COLS as f32 * TILE_SIZE; // Unused
            let grid_height_px = ROWS as f32 * TILE_SIZE;
            let start_x = -SCREEN_WIDTH / 2.0 + TILE_SIZE / 2.0 + 50.0;
            let start_y = -grid_height_px / 2.0 + TILE_SIZE / 2.0;

            // Manual inverse mapping due to start_x offset
            let grid_base_x = start_x - TILE_SIZE / 2.0;
            let grid_base_y = start_y - TILE_SIZE / 2.0;

            let col = ((world_position.x - grid_base_x) / TILE_SIZE).floor() as i32;
            let row = ((world_position.y - grid_base_y) / TILE_SIZE).floor() as i32;

            if (0..COLS).contains(&col) && (0..ROWS).contains(&row) {
                // Check occupancy
                let occupied_plant = existing_plants
                    .iter()
                    .find(|(_, cell)| cell.x == col && cell.y == row);

                match game_state.selected_tool {
                    Tool::Plant(plant_type) => {
                        if occupied_plant.is_some() {
                            return;
                        }

                        // Check cost
                        let cost = match plant_type {
                            PlantType::Peashooter => COST_PEASHOOTER,
                            PlantType::Sunflower => COST_SUNFLOWER,
                            PlantType::WallNut => COST_WALLNUT,
                            PlantType::PotatoMine => COST_POTATOMINE,
                        };

                        if game_state.sun >= cost {
                            game_state.sun -= cost;
                            // Center of cell
                            let pos_x = (col as f32).mul_add(TILE_SIZE, start_x);
                            let pos_y = (row as f32).mul_add(TILE_SIZE, start_y);
                            spawn_plant(&mut commands, plant_type, pos_x, pos_y, col, row);
                        }
                    }
                    Tool::Shovel => {
                        if let Some((entity, _)) = occupied_plant {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}
