use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::systems::spawning::spawn_plant;
use bevy::prelude::*;

pub fn button_system(
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

pub fn input_system(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut game_state: ResMut<GameState>,
    existing_plants: Query<&GridCell, With<Plant>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = match camera_q.get_single() {
            Ok(c) => c,
            Err(_) => return,
        };
        let window = match windows.get_single() {
            Ok(w) => w,
            Err(_) => return,
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
