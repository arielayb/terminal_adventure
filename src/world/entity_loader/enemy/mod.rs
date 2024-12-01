use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component, Debug)]
pub struct Enemy;

#[derive(Default, Component, Debug)]
pub struct EnemyName{
    pub enemy_name: String
}

impl EnemyName {
    pub fn set_enemy_name(&mut self, name: String) {
        self.enemy_name = name;
    }

    pub fn get_enemy_name(self) -> String {
        return self.enemy_name;
    }
}

#[derive(Default, Component, Debug)]
pub struct EnemyHealth{
    pub enemy_hp: u32
}

#[derive(Default, Component, Debug)]
pub struct EnemyTech{
    pub enemy_tp: u32
}

#[derive(Default, Component, Debug)]
pub struct EnemyLuck{
    pub enemy_lp: u32
}

#[derive(Default, Component, Debug)]
pub struct EnemyCharm{
    pub enemy_charm: u32
}

#[derive(Default, Component, Debug)]
pub struct EnemyAgility{
    pub enemy_ap: u32
}

#[derive(Default, Component, Debug)]
pub struct EnemyWeaponExp{
    pub enemy_wexp: u32
}

#[derive(Default, Component, Debug)]
pub struct EnemyLevel{
    pub enemy_lvl: u32
}

#[derive(Default, Component, Debug)]
pub struct EnemyStr{
    pub enemy_sp: u32
}

#[derive(Default, Component, Debug)]
pub struct EnemyClass{
    pub enemy_class: String
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    pub enemy_entity: Enemy,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

#[derive(Default, Component, Debug)]
pub struct EnemyEvents{
   pub interact: bool,
}
