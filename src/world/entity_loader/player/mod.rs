use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

// This plugin will contain the game.
#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Component, Debug)]
pub struct PlayerEvents{
   pub interact: bool,
}