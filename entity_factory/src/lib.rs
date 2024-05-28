/**
 * An Abstract factory class for entities/player
 */
use bevy::prelude::*;

pub trait AbstractPlayerEntity {
    fn player_entity(&self, name: String) -> bevy::ecs::component;
}

pub trait AbstractNpcEntity {
    fn npc_entity(&self, name: String) -> bevy::ecs::component;
}

pub trait AbstractEnemyEntity {
    fn enemy_enity(&self, name: String) -> bevy::ecs::component;
}

// Dynamic abstract factory using Box pointer
pub trait AbstractEntityFactory {
    fn player_entity(&self) -> Box<dyn AbstractPlayerEntity>;
    fn npc_entity(&self) -> Box<dyn AbstractNpcEntity>;
    fn enemy_enity(&self) -> Box<dyn AbstractEnemyEntity>;
}

pub struct EntityFactory {}

impl AbstractEntityFactory for EntityFactory {
    fn player_entity(&self) -> Box<dyn AbstractPlayerEntity> {
        return Box::new(NameText {});
    }

    fn npc_entity(&self) -> Box<dyn AbstractNpcEntity> {
        return Box::new(TextBox {});
    }

    fn enemy_enity(&self) -> Box<dyn AbstractEnemyEntity>{
        return Box::new(TypeDialogue {});
    }

}

struct NameText {}

impl AbstractNameTextDialogue for NameText{
    fn name_dialogue(&self, name: String) -> String {
        return String::from(name);
    }
}

struct TextBox {}

impl AbstractTextBoxDialogue for TextBox{
    fn textbox_dialogue(&self, textbox: String) -> String {
        return String::from(textbox);
    }
}

struct TypeDialogue {}

impl AbstractTypeDialogue for TypeDialogue{
    fn type_dialogue(&self, type_dialogue: String) -> String {
        return String::from(type_dialogue);
    }
}
