use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component, Debug)]
pub struct Player;

#[derive(Default, Component, Debug)]
pub struct PlayerName{
    pub player_name: String
}

#[derive(Default, Component, Debug)]
pub struct PlayerHealth{
    pub player_hp: u32
}

#[derive(Default, Component, Debug)]
pub struct PlayerTech{
    pub player_tp: u32
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