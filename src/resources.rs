use crate::components::{PlantType, Tool};
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub sun: u32,
    pub selected_tool: Tool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            sun: 150,
            selected_tool: Tool::Plant(PlantType::Peashooter),
        }
    }
}

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);
