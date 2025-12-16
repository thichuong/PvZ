use bevy::prelude::*;

pub const TILE_SIZE: f32 = 80.0;
pub const ROWS: i32 = 5;
pub const COLS: i32 = 9;
pub const SCREEN_WIDTH: f32 = COLS as f32 * TILE_SIZE + 200.0; // Extra width for HUD
pub const SCREEN_HEIGHT: f32 = ROWS as f32 * TILE_SIZE + 100.0;

// Colors
pub const COLOR_GRASS_1: Color = Color::rgb(0.0, 0.4, 0.0);
pub const COLOR_GRASS_2: Color = Color::rgb(0.0, 0.35, 0.0);
pub const COLOR_SUN_TEXT: Color = Color::WHITE;
// pub const COLOR_SELECTED_TEXT: Color = Color::GOLD; // Unused in main.rs but was there

// Plant Colors
pub const COLOR_PEASHOOTER_STEM: Color = Color::rgb(0.0, 0.3, 0.0);
pub const COLOR_PEASHOOTER_HEAD: Color = Color::rgb(0.2, 0.8, 0.2);
pub const COLOR_PEASHOOTER_SNOUT: Color = Color::rgb(0.1, 0.6, 0.1);

pub const COLOR_SUNFLOWER_STEM: Color = Color::rgb(0.0, 0.3, 0.0);
pub const COLOR_SUNFLOWER_PETALS: Color = Color::rgb(1.0, 1.0, 0.0);
pub const COLOR_SUNFLOWER_FACE: Color = Color::rgb(0.4, 0.2, 0.0);

pub const COLOR_WALLNUT_BODY: Color = Color::rgb(0.6, 0.4, 0.2);
pub const COLOR_WALLNUT_FACE: Color = Color::BLACK;

pub const COLOR_POTATOMINE_BODY: Color = Color::rgb(0.5, 0.4, 0.3);
pub const COLOR_POTATOMINE_ARMED: Color = Color::RED;

// Zombie Colors
pub const COLOR_ZOMBIE_LEGS: Color = Color::rgb(0.2, 0.2, 0.2);
pub const COLOR_ZOMBIE_BODY: Color = Color::rgb(0.2, 0.2, 0.6);
pub const COLOR_ZOMBIE_HEAD: Color = Color::rgb(0.6, 0.7, 0.6);
pub const COLOR_ZOMBIE_ARM: Color = Color::rgb(0.2, 0.2, 0.6);

pub const COLOR_BULLET: Color = Color::rgb(0.0, 1.0, 1.0);

pub const ZOMBIE_SPEED: f32 = 20.0;
pub const BULLET_SPEED: f32 = 200.0;
pub const ZOMBIE_EAT_DPS: f32 = 20.0; // Damage per second when eating

// Costs
pub const COST_PEASHOOTER: u32 = 100;
pub const COST_SUNFLOWER: u32 = 50;
pub const COST_WALLNUT: u32 = 50;
pub const COST_POTATOMINE: u32 = 25;
