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