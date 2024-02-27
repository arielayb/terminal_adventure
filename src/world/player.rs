use std::collections::binary_heap::Iter;

use crate::states::*;
use bevy::{prelude::*, utils::hashbrown::Equivalent};
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
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
    mut keys: Res<Input<KeyCode>>,
    mut key_evr: EventReader<KeyboardInput>,
    mut aseprites: ParamSet<(Query<(&mut AsepriteAnimation, &mut Transform), With<PlayerTag>>,)>,
) {
    let mut direction = Vec2::ZERO;
    for (mut player_anim, mut pos) in aseprites.p0().iter_mut() {
        // TODO: add other key/controls for player
        for ev in key_evr.iter() {
            match ev.state {
                ButtonState::Pressed => {
                    println!("Key press: {:?} ({})", ev.key_code , ev.scan_code);
                    // let key = ev.key_code.expect("W");

                    // if key == KeyCode::W {
                    //     println!("W was pressed!");
                    //     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP);
                    // }
                }
                ButtonState::Released => {
                    println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
                    // let key = ev.key_code.expect("W");

                    // if key == KeyCode::W {
                    //     println!("W was released!");
                    //     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP_IDLE);
                    // }
                }
            }
        }

        if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
            if keys.any_just_pressed([KeyCode::W, KeyCode::Up]) {
                *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP);
            }
            direction.y += 1.;
        } 
        else if keys.any_just_pressed([KeyCode::W, KeyCode::Up]) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP_IDLE);
        }
        
        if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
            if keys.any_just_pressed([KeyCode::A, KeyCode::Left]) {
                *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT);
            }
            direction.x -= 1.;
        } 
        else if keys.any_just_pressed([KeyCode::A, KeyCode::Left])  {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT_IDLE);
        }

        if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
            if keys.any_just_pressed([KeyCode::D, KeyCode::Right]) {
                *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT);
            }
            direction.x += 1.;
        } 
        else if keys.any_just_released([KeyCode::D, KeyCode::Right]) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT_IDLE);
        }

        if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
            if keys.any_just_pressed([KeyCode::S, KeyCode::Down]) {
                *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN);
            }
            direction.y -= 1.;
        } 
        else if keys.any_just_released([KeyCode::S, KeyCode::Down]) {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN_IDLE);
        }

        if direction == Vec2::ZERO {
            return;
        }

        let move_speed = 45.;
        let move_delta = direction * move_speed * time.delta_seconds();

        pos.translation += move_delta.extend(0.);
    }
}
