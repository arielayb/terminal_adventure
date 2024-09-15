use crate::states::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod entity_loader;

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

// This plugin will contain the game.
pub struct World;

impl Plugin for World {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Running), world_setup)
            .add_plugins(entity_loader::EntityLoader)
            .add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .add_systems(OnExit(GameState::Running), despawn_screen::<OnGameScreen>);
    }
}
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("terminal_world.ldtk"),
    });
}
