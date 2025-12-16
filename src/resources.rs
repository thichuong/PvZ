use crate::components::PlantType;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub sun: u32,
    pub selected_plant: PlantType,
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
pub struct SpawnTimer(pub Timer);
