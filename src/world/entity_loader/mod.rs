// use std::collections::binary_heap::Iter;
use crate::states::*;
use rand::prelude::*;
use std::collections::HashSet;
use std::{thread, time::Duration};
use bevy::{prelude::*, utils::hashbrown::Equivalent};
use bevy_ecs_ldtk::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepritePlugin};

mod player;
mod npc;

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

// This plugin will contain the game.
#[derive(Default, Component)]
pub struct EntityLoader;

impl Plugin for EntityLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spawn_player, spawn_npc))
            .add_plugins(AsepritePlugin)
            .register_ldtk_entity::<player::PlayerBundle>("Player")
            .register_ldtk_entity::<npc::NpcBundle>("NPC")
            .register_ldtk_int_cell_for_layer::<WallBundle>("Walls", 1)
            .init_resource::<LevelWalls>()
            .add_systems(Update, (move_player_from_input, move_npc, translate_grid_coords_entities, cache_wall_locations))
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnGameScreen>);
    }
}

#[derive(Default, Resource)]
struct LevelWalls {
    wall_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelWalls {
    fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}


#[derive(Resource)]
pub struct NpcWalkConfig {
    /// How often to make the npc move? (repeating timer)
    pub timer: Timer,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) 
{
    // commands
    //     .spawn(AsepriteBundle {
    //         aseprite: asset_server.load("workers/workers1.aseprite"),
    //         animation: AsepriteAnimation::from("player_down_idle"),
    //         transform: Transform {
    //             scale: Vec3::splat(1.),
    //             translation: Vec3::new(0., 80., 0.),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(PlayerTag);

    commands.spawn(
        player::PlayerBundle{
            player: player::Player,
            ..Default::default()
        }).insert(player::Player);
}


fn spawn_npc(mut commands: Commands, 
    time: Res<Time>,
    asset_server: Res<AssetServer>) 
{
    commands.spawn(
        npc::NpcBundle{
            npc: npc::Npc,
            ..Default::default()
        }
    ).insert(npc::Npc);
}

fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<(player::Player)>>,
    mut input: Res<Input<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    let movement_direction = if input.just_pressed(KeyCode::W) {
        GridCoords::new(0, 1)
    } else if input.just_pressed(KeyCode::A) {
        GridCoords::new(-1, 0)
    } else if input.just_pressed(KeyCode::S) {
        GridCoords::new(0, -1)
    } else if input.just_pressed(KeyCode::D) {
        GridCoords::new(1, 0)
    } else {
        return;
    };

    for mut player_grid_coords in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction;
        if !level_walls.in_wall(&destination) {
            *player_grid_coords = destination;
        }
    }
}

fn move_npc(
    time: Res<Time>,
    mut npc: Query<&mut GridCoords, With<npc::Npc>>,
    level_walls: Res<LevelWalls>,
) {
    let mut rng = thread_rng();
    
    let x: i32 = rng.gen_range(-1..=1);
    let y: i32  = rng.gen_range(-1..=1);
 
    // // tick the timer
    // let mut config = Timer::new(Duration::from_secs(10), TimerMode::Repeating);
    // config.tick(time.delta());
    // let movement_direction = if input.just_pressed(KeyCode::W) {
    //let mut config = Timer::from_seconds(5.0, TimerMode::Repeating);
    
    let movement_direction = GridCoords::new(x, y);

    for mut npc_grid_coords in npc.iter_mut() {
        // tick the timer
        // if config.timer.finished() {
           let destination = *npc_grid_coords + movement_direction;
            if !level_walls.in_wall(&destination) {
                *npc_grid_coords = destination;
            }
        // }
    }
}

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    const GRID_SIZE: i32 = 16;
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);
    }
}

fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut level_events: EventReader<LevelEvent>,
    walls: Query<&GridCoords, With<Wall>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    const GRID_SIZE: i32 = 16;
    for level_event in level_events.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single())
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let wall_locations = walls.iter().copied().collect();

            let new_level_walls = LevelWalls {
                wall_locations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_walls = new_level_walls;
        }
    }
}

// Tests for the abstract factory dialogue class
#[cfg(test)]
mod test{
    use entity_factory::*;

    #[test]
    fn test_init_player_entity_factory(){
        let entity_fact = EntityFactory {};
        let player_factory = AbstractEntityFactory::create_player_entity(&entity_fact, String::from("ariel")); 
        
        let player = player_factory.player_entity(String::from("ariel"), 10, 5);
        let player_ent = PlayerEntity{name: String::from("ariel"), health: 10, tech: 5};

        assert_eq!(&player.name, &player_ent.name);
        assert_eq!(&player.health, &player_ent.health);
        assert_eq!(&player.tech, &player_ent.tech);
    }

    #[test]
    fn test_init_npc_entity_factory(){
        let entity_fact = EntityFactory {};
        let npc_factory = AbstractEntityFactory::create_npc_entity(&entity_fact, String::from("Bob")); 
        
        let npc = npc_factory.npc_entity(String::from("Bob"), 10);
        // let npc = npc_factory.npc_entity(String::from("npc1"));
        // let enemey = enemy_factory.enemy_entity(String::from("enemy1"));

        let npc_ent = NpcEntity{name: String::from("Bob"), health: 10};

        assert_eq!(&npc.name, &npc_ent.name);
        assert_eq!(&npc.health, &npc_ent.health);
    }
    // #[test]
    // fn test_init_npc_entity_factory(){
    //     let entity_fact = EntityFactory {};
    //     let npc_factory = AbstractEntityFactory::create_npc_entity(&entity_fact, String::from("Bob")); 
        
    //     let npc = npc_factory.npc_entity(String::from("Bob"), 10);
    //     // let npc = npc_factory.npc_entity(String::from("npc1"));
    //     // let enemey = enemy_factory.enemy_entity(String::from("enemy1"));

    //     let npc_ent = NpcEntity{name: String::from("Bob"), health: 10};

    //     assert_eq!(&npc.name, &npc_ent.name);
    //     assert_eq!(&npc.health, &npc_ent.health);
    // }

    // #[test]
    // fn test_init_enemy_entity_factory(){
    //     let entity_fact = EntityFactory {};
    //     let enemy_factory = AbstractEntityFactory::create_enemy_entity(&entity_fact, String::from("bandit"));
        
    //     let enemy = enemy_factory.enemy_entity(String::from("bandit"), 10, 5);

    //     let enemy_ent = EnemyEntity{name: String::from("bandit"), health: 10, tech: 5};

    //     assert_eq!(&enemy.name, &enemy_ent.name);
    //     assert_eq!(&enemy.health, &enemy_ent.health);
    //     assert_eq!(&enemy.tech, &enemy_ent.tech);
    // }
}
