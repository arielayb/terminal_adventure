use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component, Debug)]
pub struct Player;

#[derive(Default, Component, Debug)]
pub struct PlayerName{
    pub player_name: String
}

impl PlayerName {
    pub fn set_player_name(&mut self, name: String) {
        self.player_name = name;
    }

    pub fn get_player_name(self) -> String {
        return self.player_name;
    }
}

#[derive(Default, Component, Debug)]
pub struct PlayerHealth{
    pub player_hp: u32
}

impl PlayerHealth {
    pub fn set_hp(&mut self, hp_val: u32) {
        println!("Health value, {:?}", &self.player_hp);
        self.player_hp = hp_val;
    }

    pub fn get_hp(&mut self) -> &u32 {
        return &self.player_hp;
    }
}

#[derive(Default, Component, Debug)]
pub struct PlayerTech{
    pub player_tp: u32
}

impl PlayerTech {
    pub fn set_tp(&mut self, tp_val: u32) {
        println!("Tech value, {:?}", &self.player_tp);
        self.player_tp = tp_val;
    }

    pub fn get_tp(&mut self) -> &u32 {
        return &self.player_tp;
    }
}

#[derive(Default, Component, Debug)]
pub struct PlayerLuck{
    pub player_lp: u32
}

#[derive(Default, Component, Debug)]
pub struct PlayerCharm{
    pub player_charm: u32
}

#[derive(Default, Component, Debug)]
pub struct PlayerAgility{
    pub player_ap: u32
}

#[derive(Default, Component, Debug)]
pub struct PlayerWeaponExp{
    pub player_wexp: u32
}

#[derive(Default, Component, Debug)]
pub struct PlayerLevel{
    pub player_lvl: u32
}

#[derive(Default, Component, Debug)]
pub struct PlayerZodiac{
    pub player_zodiac: String
}

#[derive(Default, Component, Debug)]
pub struct PlayerStr{
    pub player_sp: u32
}

impl PlayerStr {
    pub fn set_str(&mut self, str_val: u32) {
        println!("Strength value, {:?}", &self.player_sp);
        self.player_sp = str_val;
    }

    pub fn get_str(&mut self) -> &u32 {
        return &self.player_sp;
    }
}

#[derive(Default, Component, Debug)]
pub struct PlayerDef{
    pub equip_defense_val: u32
}

impl PlayerDef {
    pub fn set_defense_equip(&mut self, equip_val: u32) {
        println!("Defense value, {:?}", &self.equip_defense_val);
        self.equip_defense_val = equip_val;
    }

    pub fn get_defense_equip(&mut self) -> &u32 {
        return &self.equip_defense_val;
    }
}

#[derive(Default, Component, Debug)]
pub struct PlayerClass{
    pub player_class: String
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player_entity: Player,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

#[derive(Default, Component, Debug)]
pub struct PlayerEvents{
   pub interact: bool,
}
