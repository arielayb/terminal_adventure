use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::time::Duration;

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

// #[derive(Resource)]
// pub struct NpcWalkConfig {
//     /// How often to make the npc move? (repeating timer)
//     pub timer: Timer,
// }

