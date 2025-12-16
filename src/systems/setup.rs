use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
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
    // let panel_x = SCREEN_WIDTH / 2.0 - 90.0; // Unused

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
