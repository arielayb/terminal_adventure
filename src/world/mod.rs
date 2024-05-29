use crate::states::*;
use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepritePlugin};
use bevy_ecs_ldtk::prelude::*;

mod player;
mod non_playable_char;
mod abstract_world_builder;
mod farm;

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

// This plugin will contain the game.
pub struct World;

impl Plugin for World {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), world_setup)
            .add_plugins(player::Player)
            .add_plugins(non_playable_char::Npc)
            .add_plugins(LdtkPlugin)
            // .register_ldtk_entity::<PlayerBundle>("Player")
            .insert_resource(LevelSelection::index(0))
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnGameScreen>);
    }
}

fn world_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //commands.spawn(Camera2dBundle::default());
    
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("terminal_world.ldtk"),
        ..Default::default()
    });
}
