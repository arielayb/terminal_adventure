use crate::states::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;

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

// setup the world and camera
fn world_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //commands.spawn(Camera2dBundle::default());
    
    // let mut camera = Camera2dBundle::default();
    // camera.projection.scale = 0.5;
    // camera.transform.translation.x += 1280.0 / 4.0;
    // camera.transform.translation.y += 720.0 / 4.0;
    // camera.camera.hdr = true;

    // commands.spawn(camera);
  
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("terminal_world.ldtk"),
        ..Default::default()
    });
}
