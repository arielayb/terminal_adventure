use crate::states::*;
use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepritePlugin};

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

mod sprites {
    use bevy_aseprite::aseprite;
    aseprite!(pub Player, "workers/workers1.aseprite");
}

#[derive(Component, Clone, Copy, Debug)]
struct PlayerTag;

// This plugin will contain the game.
pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_plugins(AsepritePlugin)
            .add_systems(Update, player_movement)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnGameScreen>);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(AsepriteBundle {
            aseprite: asset_server.load("workers/workers1.aseprite"),
            animation: AsepriteAnimation::from("player_down_idle"),
            transform: Transform {
                scale: Vec3::splat(1.),
                translation: Vec3::new(0., 80., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerTag);
}

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut aseprites: ParamSet<(Query<(&mut AsepriteAnimation, &mut Transform), With<PlayerTag>>,)>,
) {
    let mut direction = Vec2::ZERO;
    for (mut player_anim, mut pos) in &mut aseprites.p0().iter_mut() {
        if keys.pressed(KeyCode::W) {
            if keys.just_pressed(KeyCode::W) {
                *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP);
            }
            direction.y += 1.;

        } else if keys.just_released(KeyCode::W) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP_IDLE);
        }

        if keys.pressed(KeyCode::A) {
            if keys.just_pressed(KeyCode::A) {
                *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT);
            }
            direction.x -= 1.;

        } else if keys.just_released(KeyCode::A) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT_IDLE);
        }

        if keys.pressed(KeyCode::D) {
            if keys.just_pressed(KeyCode::D) {
                *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT);
            }
            direction.x += 1.;

        } else if keys.just_released(KeyCode::D) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT_IDLE);
        }

        if keys.pressed(KeyCode::S) {
            if keys.just_pressed(KeyCode::S) {
                *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN);
            }
            direction.y -= 1.;

        } else if keys.just_released(KeyCode::S) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN_IDLE);
        }

        let move_speed = 45.;
        let move_delta = direction * move_speed * time.delta_seconds();

        pos.translation += move_delta.extend(0.);
    }
}