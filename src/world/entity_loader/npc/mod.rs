use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::vec::Vec;

// This plugin will contain the NPC or events.
#[derive(Default, Component, Clone)]
pub struct Npc;

#[derive(Default, Component, Clone, Debug)]
pub struct NpcName{
    pub npc_name: String
}

#[derive(Default, Component, Clone, Debug)]
pub struct NpcHealth{
    pub npc_hp: u32
}

#[derive(Default, Component, Clone, Debug)]
pub struct NpcDialogue{
    pub dialogue: String 
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct NpcBundle {
    pub npc: Npc,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

#[derive(Resource, Debug)]
pub struct NpcWalkConfig {
    /// How often to make the npc move? (repeating timer)
    pub walk_timer: Timer,
}

impl NpcWalkConfig {
    pub fn new() -> Self {
        Self {
            walk_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

impl Default for NpcWalkConfig {
    fn default() -> Self {
        Self::new()
    }
}
