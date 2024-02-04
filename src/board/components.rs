use crate::board::vectors::Vector2Int;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Position {
    pub v: Vector2Int,
}

#[derive(Component)]
pub struct Tile;

#[derive(Default, Resource)]
pub struct BoardRes {
    pub tiles: HashMap<Vector2Int, Entity>,
}
