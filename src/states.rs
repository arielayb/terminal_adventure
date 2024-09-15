use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    TitleMenu,
    Running,
    LoadAssets,
}

// #[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
// #[source(GameState = GameState::Running)
#[derive(Debug, Default, States, Clone, Eq, PartialEq, Hash)]
pub enum PausedState {
    #[default]
    Paused,
    Unpaused,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
