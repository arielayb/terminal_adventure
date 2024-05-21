use crate::states::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepritePlugin};

mod player;
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
            .add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnGameScreen>);
    }
}

fn world_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("terminal_world.ldtk"),
        ..Default::default()
    });
    // commands
    //     .spawn(AsepriteBundle {
    //         aseprite: asset_server.load("workers/workers1.aseprite"),
    //         animation: AsepriteAnimation::from("player_down_idle"),
    //         transform: Transform {
    //             scale: Vec3::splat(1.),
    //             translation: Vec3::new(0., 80., 0.),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(PlayerTag);
}
