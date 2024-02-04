// board/systems.rs
use super::components::{Position, Tile};
use super::CurrentBoard;
use crate::board::vectors::Vector2Int;
use bevy::prelude::*;
use std::collections::HashMap;
//mod vectors;

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentBoard>) {
    current.tiles = HashMap::new();
    for x in 0..8 {
        for y in 0..8 {
            let v = Vector2Int::new(x, y);
            let tile = commands.spawn((Position { v }, Tile)).id();
            current.tiles.insert(v, tile);
        }
    }
}
