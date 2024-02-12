use crate::states::*;
use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepritePlugin};

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Component, Clone, Copy, Debug)]
struct PlayerTag;

#[derive(Component)]
enum PlayerMovement {
    Up_idle,
    Down_idle,
    Left_idle,
    Right_idle,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

mod sprites {
    use bevy_aseprite::aseprite;
    aseprite!(pub Player, "workers/workers1.aseprite");
}

// This plugin will contain the game.
pub struct World;

impl Plugin for World {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), world_setup)
            .add_plugins(AsepritePlugin)
            .add_systems(Update, player_movement)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnGameScreen>);
    }
}

fn world_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((AsepriteBundle {
            aseprite: asset_server.load("workers/workers1.aseprite"),
            animation: AsepriteAnimation::from("player_left"),
            transform: Transform {
                scale: Vec3::splat(1.),
                translation: Vec3::new(0., 80., 0.),
                ..Default::default()
            },
            ..Default::default()
        },PlayerMovement::Down_idle,))
        .insert(PlayerTag);
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut aseprites: ParamSet<(Query<&mut AsepriteAnimation, With<PlayerTag>>,)>,
) {
    for mut player_anim in aseprites.p0().iter_mut() {
        if keys.pressed(KeyCode::W) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP);
        }

        if keys.pressed(KeyCode::A) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT);
        }

        if keys.pressed(KeyCode::D) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT);
        }else if keys.just_released(KeyCode::D) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT_IDLE);
        }

        if keys.just_pressed(KeyCode::S) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN);
        
           
        }else if keys.just_released(KeyCode::S) {
                *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN_IDLE);
        }
    }
}
