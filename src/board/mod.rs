// board/mod.rs
use bevy::prelude::*;
use std::collections::HashMap;

use crate::states::GameState;
//use crate::vectors::Vector2Int;

pub mod components;
mod systems;
mod vectors;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .add_system(systems::spawn_map.in_schedule(OnEnter(GameState::Playing)));
    }
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>,
}
