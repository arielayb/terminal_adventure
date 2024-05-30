/**
 * An Abstract factory class for entities/player
 */
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub trait AbstractPlayerEntity {
    fn player_entity(&self, name: String, hp: i32, tp: i32, mut commands: Commands);
}

pub trait AbstractNpcEntity {
    fn npc_entity(&self, name: String, hp: i32, mut commands: Commands);
}

pub trait AbstractEnemyEntity {
    fn enemy_enity(&self, name: String, hp: i32, tp: i32, mut commands: Commands);
}

// Dynamic abstract factory using Box pointer
pub trait AbstractEntityFactory {
    fn create_player_entity(&self, player_name: String) -> Box<dyn AbstractPlayerEntity>;
    fn create_npc_entity(&self, npc_name: String) -> Box<dyn AbstractNpcEntity>;
    fn create_enemy_enity(&self, enemy_name: String) -> Box<dyn AbstractEnemyEntity>;
}

pub struct EntityFactory {}

impl AbstractEntityFactory for EntityFactory {
    fn create_player_entity(&self, player_name: String) -> Box<dyn AbstractPlayerEntity> {
        return Box::new(PlayerEntity {name: player_name, health: 10, tech: 0});
    }

    fn create_npc_entity(&self, npc_name: String) -> Box<dyn AbstractNpcEntity> {
        return Box::new(NpcEntity {name: npc_name, health: 10});
    }

    fn create_enemy_enity(&self, enemy_name: String) -> Box<dyn AbstractEnemyEntity>{
        return Box::new(EnemyEntity {name: enemy_name, health: 10, tech: 0});
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: PlayerEntity,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Component, Clone, Debug)]
struct PlayerEntity {
    name: String,
    health: i32,
    tech: i32,
}

impl AbstractPlayerEntity for PlayerEntity{
    fn player_entity(&self, player_name: String, hp: i32, tp: i32) {
        commands.spawn(
    PlayerBundle{
                PlayerEntity{
                    name: player_name,
                    health: hp,
                    tech: tp,
            },
            ..Default::default()
        });
    }
}

#[derive(Component, Clone, Debug)]
struct NpcEntity {
    name: String,
    health: i32,
}

impl AbstractNpcEntity for NpcEntity{
    fn npc_entity(&self, npc_name: String, hp: i32, mut commands: Commands) {

    }
}

#[derive(Component, Clone, Debug)]
struct EnemyEntity {
    name: String,
    health: i32,
    tech: i32,
}

impl AbstractEnemyEntity for EnemyEntity{
    fn enemy_enity(&self, enemy_name: String, hp: i32, tp: i32, mut commands: Commands) {
        
    }
}
