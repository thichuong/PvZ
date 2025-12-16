use crate::components::SunText;
use crate::resources::GameState;
use bevy::prelude::*;

pub fn ui_system(game_state: Res<GameState>, mut sun_query: Query<&mut Text, (With<SunText>)>) {
    for mut text in sun_query.iter_mut() {
        text.sections[0].value = format!("Sun: {}", game_state.sun);
    }
}
