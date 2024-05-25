// use std::collections::binary_heap::Iter;

use crate::states::*;
use bevy::{prelude::*, utils::hashbrown::Equivalent};
use bevy_ecs_ldtk::prelude::*;
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

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Component, Clone, Copy, Debug)]
struct PlayerTag;

// // This plugin will contain the game.
#[derive(Default, Component)]
pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_plugins(AsepritePlugin)
            .register_ldtk_entity::<PlayerBundle>("Player")
            // .add_systems(Update, player_movement)
            .add_systems(Update, move_player_from_input)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnGameScreen>);
    }
}

fn move_player_from_input(
    time: Res<Time>,
    mut players: Query<&mut Transform, With<Player>>,
    mut keys: Res<Input<KeyCode>>,
) {
    //2D vector for player's direction
    let mut direction = Vec2::ZERO;
    
    // key codes for each direction
    const KEYS_UP: [KeyCode; 2] = [KeyCode::Up, KeyCode::W];
    const KEYS_DOWN: [KeyCode; 2] = [KeyCode::Down, KeyCode::S];
    const KEYS_LEFT: [KeyCode; 2] = [KeyCode::Left, KeyCode::A];
    const KEYS_RIGHT: [KeyCode; 2] = [KeyCode::Right, KeyCode::D];

    for mut pos in players.iter_mut() { 
        // if keys.any_just_released(KEYS_UP) && !keys.any_pressed(KEYS_UP) 
        //     && !keys.any_pressed(KEYS_LEFT) && !keys.any_pressed(KEYS_RIGHT) {
        //     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP_IDLE);
        // } else if keys.any_just_released(KEYS_LEFT) && !keys.any_pressed(KEYS_UP) 
        //     && !keys.any_pressed(KEYS_DOWN) && !keys.any_pressed(KEYS_RIGHT) {
        //     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT_IDLE);
        // } else if keys.any_just_released(KEYS_RIGHT) && !keys.any_pressed(KEYS_UP) 
        //     && !keys.any_pressed(KEYS_DOWN) && !keys.any_pressed(KEYS_LEFT){
        //     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT_IDLE);
        // } if keys.any_just_released(KEYS_DOWN) && !keys.any_pressed(KEYS_DOWN) 
        //     && !keys.any_pressed(KEYS_LEFT) && !keys.any_pressed(KEYS_RIGHT) {
        //     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN_IDLE);
        // } 

        if keys.any_pressed(KEYS_LEFT) && !keys.any_pressed(KEYS_RIGHT) 
            && !keys.any_just_pressed(KEYS_RIGHT){
            // if keys.any_just_pressed(KEYS_LEFT) {    
            //     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT);
            // }else if keys.any_pressed(KEYS_UP) {
            //     if keys.any_just_pressed(KEYS_UP) {
            //         *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP);
            //     }
            // }else if keys.any_pressed(KEYS_DOWN) {
            //     if keys.any_just_pressed(KEYS_DOWN) && keys.any_just_pressed(KEYS_LEFT) {
            //         *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN);
            //     }
            // }
            direction.x -= 1.;
        }else if keys.any_pressed(KEYS_RIGHT) && !keys.any_pressed(KEYS_LEFT) 
                && !keys.any_just_pressed(KEYS_LEFT){
            // if keys.any_just_pressed(KEYS_RIGHT) {
            //     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT);
            // }else if keys.any_pressed(KEYS_UP) {
            //     if keys.any_just_pressed(KEYS_UP) {
            //         *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP);
            //     }
            // }else if keys.any_pressed(KEYS_DOWN) {
            //     if keys.any_just_pressed(KEYS_DOWN) {
            //         *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN);
            //     }
            // }
            direction.x += 1.;
        } 

        if keys.any_pressed(KEYS_UP) && !keys.any_pressed(KEYS_DOWN) 
            && !keys.any_just_pressed(KEYS_LEFT) && !keys.any_just_pressed(KEYS_RIGHT) {
            // if keys.any_just_pressed(KEYS_UP) {
            //     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP);
            // }else if keys.any_pressed(KEYS_LEFT) {
            //     if keys.any_just_pressed(KEYS_LEFT) {
            //         *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT);
            //     }
            // }else if keys.any_pressed(KEYS_RIGHT) {
            //     if keys.any_just_pressed(KEYS_RIGHT) {
            //         *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT);
            //     } 
            // }
            direction.y += 1.;
        }else if keys.any_pressed(KEYS_DOWN) && !keys.any_pressed(KEYS_UP) 
                && !keys.any_just_pressed(KEYS_LEFT) && !keys.any_just_pressed(KEYS_RIGHT) {
            // if keys.any_just_pressed(KEYS_DOWN) {
            //     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN);
            // }else if keys.any_pressed(KEYS_LEFT) {
            //     if keys.any_just_pressed(KEYS_LEFT) {
            //         *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT);
            //     }
            // }else if keys.any_pressed(KEYS_RIGHT) {
            //     if keys.any_just_pressed(KEYS_RIGHT) {
            //         *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT);
            //     } 
            // }
            direction.y -= 1.;
        }

        if direction == Vec2::ZERO {
            return;
        }

        let move_speed = 45.;
        let move_delta = direction * move_speed * time.delta_seconds();

        pos.translation += move_delta.extend(0.);        
    }

    // let movement_direction = if input.just_pressed(KeyCode::W) {
    //     GridCoords::new(0, 1)
    // } else if input.just_pressed(KeyCode::A) {
    //     GridCoords::new(-1, 0)
    // } else if input.just_pressed(KeyCode::S) {
    //     GridCoords::new(0, -1)
    // } else if input.just_pressed(KeyCode::D) {
    //     GridCoords::new(1, 0)
    // } else {
    //     return;
    // };

    // for mut player_grid_coords in players.iter_mut() {
    //     let destination = *player_grid_coords + movement_direction;
    //     *player_grid_coords = destination;
    // }
}

const GRID_SIZE: i32 = 16;

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    commands.spawn(PlayerBundle{
        player: Player,
        sprite_sheet_bundle: SpriteSheetBundle { 
            transform: Transform {
                scale: Vec3::splat(1.),
                translation: Vec3::new(0., 80., 0.),
                ..Default::default()
            },
            ..Default::default()},
    }).insert(PlayerTag);
}

// fn player_movement(
//     time: Res<Time>,
//     mut input: Res<Input<KeyCode>>,
//     mut key_evr: EventReader<KeyboardInput>,
//     mut players: Query<&mut GridCoords, With<Player>>,
//     mut aseprites: ParamSet<(Query<(&mut AsepriteAnimation, &mut Transform), With<PlayerTag>>,)>,
// ) {
//     // 2D vector for player's direction
//     let mut direction = Vec2::ZERO;
    
//     // key codes for each direction
//     const KEYS_UP: [KeyCode; 2] = [KeyCode::Up, KeyCode::W];
//     const KEYS_DOWN: [KeyCode; 2] = [KeyCode::Down, KeyCode::S];
//     const KEYS_LEFT: [KeyCode; 2] = [KeyCode::Left, KeyCode::A];
//     const KEYS_RIGHT: [KeyCode; 2] = [KeyCode::Right, KeyCode::D];

//     for (mut player_anim, mut pos) in aseprites.p0().iter_mut() { 
//         if keys.any_just_released(KEYS_UP) && !keys.any_pressed(KEYS_UP) 
//             && !keys.any_pressed(KEYS_LEFT) && !keys.any_pressed(KEYS_RIGHT) {
//             *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP_IDLE);
//         } else if keys.any_just_released(KEYS_LEFT) && !keys.any_pressed(KEYS_UP) 
//             && !keys.any_pressed(KEYS_DOWN) && !keys.any_pressed(KEYS_RIGHT) {
//             *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT_IDLE);
//         } else if keys.any_just_released(KEYS_RIGHT) && !keys.any_pressed(KEYS_UP) 
//             && !keys.any_pressed(KEYS_DOWN) && !keys.any_pressed(KEYS_LEFT){
//             *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT_IDLE);
//         } if keys.any_just_released(KEYS_DOWN) && !keys.any_pressed(KEYS_DOWN) 
//             && !keys.any_pressed(KEYS_LEFT) && !keys.any_pressed(KEYS_RIGHT) {
//             *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN_IDLE);
//         } 

//         if keys.any_pressed(KEYS_LEFT) && !keys.any_pressed(KEYS_RIGHT) 
//             && !keys.any_just_pressed(KEYS_RIGHT){
//             if keys.any_just_pressed(KEYS_LEFT) {    
//                 *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT);
//             }else if keys.any_pressed(KEYS_UP) {
//                 if keys.any_just_pressed(KEYS_UP) {
//                     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP);
//                 }
//             }else if keys.any_pressed(KEYS_DOWN) {
//                 if keys.any_just_pressed(KEYS_DOWN) && keys.any_just_pressed(KEYS_LEFT) {
//                     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN);
//                 }
//             }
//             direction.x -= 1.;
//         }else if keys.any_pressed(KEYS_RIGHT) && !keys.any_pressed(KEYS_LEFT) 
//                 && !keys.any_just_pressed(KEYS_LEFT){
//             if keys.any_just_pressed(KEYS_RIGHT) {
//                 *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT);
//             }else if keys.any_pressed(KEYS_UP) {
//                 if keys.any_just_pressed(KEYS_UP) {
//                     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP);
//                 }
//             }else if keys.any_pressed(KEYS_DOWN) {
//                 if keys.any_just_pressed(KEYS_DOWN) {
//                     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN);
//                 }
//             }
//             direction.x += 1.;
//         } 

//         if keys.any_pressed(KEYS_UP) && !keys.any_pressed(KEYS_DOWN) 
//             && !keys.any_just_pressed(KEYS_LEFT) && !keys.any_just_pressed(KEYS_RIGHT) {
//             if keys.any_just_pressed(KEYS_UP) {
//                 *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_UP);
//             }else if keys.any_pressed(KEYS_LEFT) {
//                 if keys.any_just_pressed(KEYS_LEFT) {
//                     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT);
//                 }
//             }else if keys.any_pressed(KEYS_RIGHT) {
//                 if keys.any_just_pressed(KEYS_RIGHT) {
//                     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT);
//                 } 
//             }
//             direction.y += 1.;
//         }else if keys.any_pressed(KEYS_DOWN) && !keys.any_pressed(KEYS_UP) 
//                 && !keys.any_just_pressed(KEYS_LEFT) && !keys.any_just_pressed(KEYS_RIGHT) {
//             if keys.any_just_pressed(KEYS_DOWN) {
//                 *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_DOWN);
//             }else if keys.any_pressed(KEYS_LEFT) {
//                 if keys.any_just_pressed(KEYS_LEFT) {
//                     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_LEFT);
//                 }
//             }else if keys.any_pressed(KEYS_RIGHT) {
//                 if keys.any_just_pressed(KEYS_RIGHT) {
//                     *player_anim = AsepriteAnimation::from(sprites::Player::tags::PLAYER_RIGHT);
//                 } 
//             }
//             direction.y -= 1.;
//         }

//         if direction == Vec2::ZERO {
//             return;
//         }

//         let move_speed = 45.;
//         let move_delta = direction * move_speed * time.delta_seconds();

//         pos.translation += move_delta.extend(0.);
//     }
// }
