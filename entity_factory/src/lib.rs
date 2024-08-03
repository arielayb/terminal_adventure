/**
 * An Abstract factory class for entities/player
 */
use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupPlugin, TextPopupTimeout};

pub trait AbstractPlayerEntity {
    fn player_entity(&self, name: String, hp: i32, tp: i32) -> PlayerEntity;
}

pub trait AbstractNpcEntity {
    fn npc_entity(&self, name: String, hp: i32) -> NpcEntity;
}

pub trait AbstractEnemyEntity {
    fn enemy_entity(&self, name: String, hp: i32, tp: i32) -> EnemyEntity;
}

// Dynamic abstract factory using Box pointer
pub trait AbstractEntityFactory {
    fn create_player_entity(&self, player_name: String) -> Box<dyn AbstractPlayerEntity>;
    fn create_npc_entity(&self, npc_name: String) -> Box<dyn AbstractNpcEntity>;
    fn create_enemy_entity(&self, enemy_name: String) -> Box<dyn AbstractEnemyEntity>;
}

pub struct EntityFactory {}

impl AbstractEntityFactory for EntityFactory {
    fn create_player_entity(&self, player_name: String) -> Box<dyn AbstractPlayerEntity> {
        return Box::new(PlayerEntity {name: player_name, health: 10, tech: 0});
    }

    fn create_npc_entity(&self, npc_name: String) -> Box<dyn AbstractNpcEntity> {
        return Box::new(NpcEntity {name: npc_name, health: 10});
    }

    fn create_enemy_entity(&self, enemy_name: String) -> Box<dyn AbstractEnemyEntity>{
        return Box::new(EnemyEntity {name: enemy_name, health: 10, tech: 0});
    }
}

#[derive(Component, Clone, Debug)]
pub struct PlayerEntity {
    pub name: String,
    pub health: i32,
    pub tech: i32,
}

impl AbstractPlayerEntity for PlayerEntity{
    fn player_entity(&self, player_name: String, hp: i32, tp: i32) -> PlayerEntity {
        let player_entity = PlayerEntity{
            name: player_name,
            health: hp,
            tech: tp,
        };

        return player_entity;
    }
}

#[derive(Component, Clone, Debug)]
pub struct NpcEntity {
    pub name: String,
    pub health: i32,
}

impl AbstractNpcEntity for NpcEntity{
    fn npc_entity(&self, npc_name: String, hp: i32) -> NpcEntity {
        let npc_entity = NpcEntity{
            name: npc_name,
            health: hp,
        };

        return npc_entity;
    }
}

#[derive(Component, Clone, Debug)]
pub struct EnemyEntity {
    pub name: String,
    pub health: i32,
    pub tech: i32,
}

impl AbstractEnemyEntity for EnemyEntity{
    fn enemy_entity(&self, enemy_name: String, hp: i32, tp: i32) -> EnemyEntity{
        let enemy_entity = EnemyEntity{
            name: enemy_name,
            health: hp,
            tech: tp,
        };

        return enemy_entity;
    }
}
