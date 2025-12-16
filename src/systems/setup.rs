use crate::components::{PlantType, SunText, Tool, ToolButton};
use crate::constants::{
    COLOR_GRASS_1, COLOR_GRASS_2, COLOR_PEASHOOTER_HEAD, COLOR_POTATOMINE_BODY,
    COLOR_SUNFLOWER_PETALS, COLOR_SUN_TEXT, COLOR_WALLNUT_BODY, COLS, COST_PEASHOOTER,
    COST_POTATOMINE, COST_SUNFLOWER, COST_WALLNUT, ROWS, SCREEN_WIDTH, TILE_SIZE,
};
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Grid Background (Left side)
    // let grid_width_px = COLS as f32 * TILE_SIZE; // Unused
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
                    (col as f32).mul_add(TILE_SIZE, start_x),
                    (row as f32).mul_add(TILE_SIZE, start_y),
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
                    top: Val::Px((i as f32).mul_add(70.0, 50.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            })
            .insert(ToolButton(Tool::Plant(*ptype)))
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
                        format!("{label}\n{cost}"),
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

    // Shovel Button
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(80.0),
                height: Val::Px(60.0),
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px((plants.len() as f32).mul_add(70.0, 50.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..default()
        })
        .insert(ToolButton(Tool::Shovel))
        .with_children(|parent| {
            // Shovel Icon Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|icon_parent| {
                    // Handle Loop/Grip
                    icon_parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(12.0),
                            height: Val::Px(4.0),
                            margin: UiRect::bottom(Val::Px(0.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.4, 0.2, 0.1).into(), // Brown
                        ..default()
                    });

                    // Shaft
                    icon_parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(4.0),
                            height: Val::Px(12.0),
                            ..default()
                        },
                        background_color: Color::rgb(0.4, 0.2, 0.1).into(), // Brown
                        ..default()
                    });

                    // Blade
                    icon_parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(16.0),
                            height: Val::Px(14.0),
                            ..default()
                        },
                        background_color: Color::SILVER.into(),
                        ..default()
                    });
                });

            parent.spawn(
                TextBundle::from_section(
                    "Shovel",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
        });
}
