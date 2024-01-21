use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    TitleMenu,
    Playing,
}