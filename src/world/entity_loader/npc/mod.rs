use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Bundle, LdtkEntity)]
pub struct NpcBundle {
    pub npc: Npc,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

// This plugin will contain the NPC or events.
#[derive(Default, Component)]
pub struct Npc;
