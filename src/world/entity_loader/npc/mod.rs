use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// This plugin will contain the NPC or events.
#[derive(Default, Component, Clone)]
pub struct Npc;

#[derive(Default, Component, Clone, Debug)]
pub struct NpcName {
    pub npc_name: String,
}

#[derive(Default, Component, Clone, Debug)]
pub struct NpcPosition {
    pub npc_position: Box<GridCoords>
}

#[derive(Default, Component, Clone, Debug)]
pub struct NpcHealth {
    pub npc_hp: u32,
}

#[derive(Default, Component, Clone, Debug)]
pub struct NpcDialogue {
    pub dialogue: String,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct NpcBundle {
    pub npc: Npc,
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

impl NpcBundle {
    pub fn get_default_coords(&mut self) -> NpcPosition {
        let npc_pos = NpcPosition {
            npc_position: Box::new(GridCoords { x: self.grid_coords.x, y: self.grid_coords.y })
        };
        
        return npc_pos;
    }
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
