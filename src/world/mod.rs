use crate::states::*;
use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepritePlugin};

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
enum PlayerMovement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

// This plugin will contain the game.
pub struct World;

impl Plugin for World {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), world_setup)
            .add_plugins(AsepritePlugin)
            .add_systems(Update, animate_sprite)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnGameScreen>);
    }
}

fn animate_sprite(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut PlayerMovement,
    )>,
) {
    for (indices, mut timer, mut sprite, mut player_movement) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn world_setup(mut commands: Commands,asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load("workers/workers1.aseprite"),
        animation: AsepriteAnimation::from("player_left"),
        transform: Transform {
            scale: Vec3::splat(1.),
            translation: Vec3::new(0., 80., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}
