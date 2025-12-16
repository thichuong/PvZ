use bevy::prelude::*;
use bevy::window::WindowResolution;

mod components;
mod constants;
mod resources;
mod systems;

use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use resources::{GameState, SpawnTimer};
use systems::{
    combat::{collision_system, explosion_damage_system, zombie_eat_system},
    gameplay::{move_bullets, move_zombies, plant_action},
    input::{button_system, input_system},
    particles::particle_system,
    setup::setup,
    spawning::spawn_zombies,
    ui::ui_system,
};

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
                explosion_damage_system,
                particle_system,
                ui_system,
            ),
        )
        .run();
}
