/*
    entity_loader module
    This module focuses on loading the entities and setting up the small
    components for NPCs and player modules.
*/
use crate::states::*;
use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::audio::CpalSample;
use bevy_text_popup::{TextPopupEvent, TextPopupPlugin, TextPopupButton, TextPopupTimeout, TextPopupLocation};
use bevy_ecs_ldtk::prelude::*;
use rand::prelude::*;
use std::collections::HashSet;
use std::ops::Index;
use name_maker::RandomNameGenerator;
use name_maker::Gender;

mod npc;
mod player;
mod enemy;
mod graph_system;
mod dice_system;

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

// This plugin will contain the game.
#[derive(Default, Component)]
pub struct EntityLoader;

impl Plugin for EntityLoader {
    fn build(&self, app: &mut App) {
        // app.configure_sets(Update, ());

        app.add_systems(OnEnter(GameState::Running), (camera_setup, spawn_player, spawn_npc))
            .register_ldtk_entity::<npc::NpcBundle>("NPC")
            .register_ldtk_entity::<player::PlayerBundle>("Player")
            .register_ldtk_int_cell_for_layer::<WallBundle>("Walls", 1)
            .register_ldtk_int_cell_for_layer::<WallBundle>("Water", 1)
            .init_resource::<LevelWalls>()
            .init_resource::<npc::NpcWalkConfig>()
            .add_plugins(TextPopupPlugin)
            .add_systems(Update, (player_control, 
                                                    move_npc, 
                                                    translate_grid_coords_entities, 
                                                    cache_wall_locations, 
                                                    npc_interact,
                                                    update_camera))
            .add_systems(OnExit(GameState::Running), despawn_screen::<OnGameScreen>);
    }
}

// setup the world and camera
fn camera_setup(mut commands: Commands) {
    //commands.spawn(Camera2dBundle::default());
    
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.3;
    camera.transform.translation.x += 640.0/4.;
    camera.transform.translation.y += 480.0/4.;
    camera.camera.hdr = true;

    commands.spawn(camera);
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

fn spawn_player(mut commands: Commands) {
    // TODO: work on removing factory design pattern in next goal
    let temp_name = String::from("ariel");
    let temp_hp: u32 = 10;
    // let temp_tp: u32 = 5;

    commands.spawn(
        (player::PlayerBundle {
            player_entity: player::Player,
            ..Default::default()
        },player::PlayerName{
            player_name: temp_name 
        },player::PlayerHealth{
            player_hp: temp_hp
        })
    ).insert(player::PlayerEvents{interact: false});
}

fn spawn_npc(mut commands: Commands) {
    // TODO: work on design patterns for random NPC generator in next goal
    // get a range for the name
    //let rng = RNG::try_from(&Language::Fantasy).unwrap();
    let rng = RandomNameGenerator::init();

    // Prints a random name with a masculine first name.
    let first_name: String = rng.generate_specific(Gender::Male).first_name;

    // let temp_hp: u32 = 10;
    let arr_dialogue: [&str; 5] = ["Hi...", "Hello..", "Yes?", "Go away.", "What do you want?"];
    let mut rng = thread_rng();
    let i: usize  = rng.gen_range(0..=4);

    commands.spawn(
        (npc::NpcBundle{
            npc: npc::Npc,
            ..Default::default()
        },npc::NpcName{
            npc_name: first_name
        }, npc::NpcDialogue{
            dialogue: arr_dialogue[i].to_string()
        })
    );
}

fn player_control(
    mut players: Query<&mut GridCoords, With<player::Player>>,
    mut player_event: Query<&mut player::PlayerEvents, With<player::PlayerEvents>>,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    if input.just_pressed(KeyCode::KeyE) {
        info!("e key pressed");
        // interaction with objects
        let mut touch = player_event.single_mut();
        touch.interact = true;
    } else if input.just_released(KeyCode::KeyE) {
        let mut touch = player_event.single_mut();
        touch.interact = false;
    }

    let movement_direction = if input.just_pressed(KeyCode::KeyW) 
    || input.just_pressed(KeyCode::Numpad8) {
        GridCoords::new(0, 1)
    } else if input.just_pressed(KeyCode::KeyA) 
    || input.just_pressed(KeyCode::Numpad4) {
        GridCoords::new(-1, 0)
    } else if input.just_pressed(KeyCode::KeyS) 
    || input.just_pressed(KeyCode::Numpad5) {
        GridCoords::new(0, -1)
    } else if input.just_pressed(KeyCode::KeyD) 
    || input.just_pressed(KeyCode::Numpad6) {
        GridCoords::new(1, 0)
    } else if input.just_pressed(KeyCode::Numpad9) {
        GridCoords::new(1, 1)
    } else if input.just_pressed(KeyCode::Numpad1) {
        GridCoords::new(-1, -1)
    } else if input.just_pressed(KeyCode::Numpad7) {
        GridCoords::new(-1, 1)
    } else if input.just_pressed(KeyCode::Numpad3) {
            GridCoords::new(1, -1)
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
    mut npc_timer: ResMut<npc::NpcWalkConfig>,
    level_walls: Res<LevelWalls>,
) {
    let mut rng = thread_rng();
    
    let x: i32 = rng.gen_range(-1..=1);
    let y: i32  = rng.gen_range(-1..=1);

    // tick the timer
    npc_timer.walk_timer.tick(time.delta());
       
    let movement_direction = GridCoords::new(x, y);

    for mut npc_grid_coords in npc.iter_mut() {
        if npc_timer.walk_timer.finished() {
           let destination = *npc_grid_coords + movement_direction;
            if !level_walls.in_wall(&destination) {
                *npc_grid_coords = destination;
            }
        }
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

fn npc_interact(
    asset_server: Res<AssetServer>,
    players: Query<&GridCoords, With<player::Player>>,
    mut text_popup_events: EventWriter<TextPopupEvent>,
    mut player_event: Query<&mut player::PlayerEvents, With<player::PlayerEvents>>,
    npc_coords: Query<&GridCoords, With<npc::Npc>>,
    mut npc_name: Query<&npc::NpcName, With<npc::NpcName>>,
    mut npc_dialogue: Query<&npc::NpcDialogue, With<npc::NpcDialogue>>

) {
    if players
        .iter()
        .zip(npc_coords.iter())
        .any(|(player_grid_coords, npc_grid_coords)| player_grid_coords == npc_grid_coords)
    {
        info!("Npc collision detected...");
        //let mut rng = thread_rng();
        let touch = player_event.single_mut();
        //let n: usize = rng.gen_range(0..=4);

        if touch.interact {
            info!("<<< NPC interaction >>>");
            // println!("{}", npc.single_mut().1.clone().get_npc_name());
            text_popup_events.send(TextPopupEvent {
                content: format!("{} : \n{}", npc_name.single_mut().npc_name.to_string(), npc_dialogue.single_mut().dialogue),
                font: Some(asset_server.load("fonts/Fortine-Regular.otf")),
                font_size: 20.,
                location: TextPopupLocation::Bottom,
                text_alignment: JustifyText::Left,
                border_color: Color::linear_rgb(100., 100., 100.),
                modal: Some(Color::linear_rgba(0., 0., 0., 0.)),
                timeout: TextPopupTimeout::Seconds(5),
                ..default()
            });
        }
    }
}

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<player::Player>)>,
    player: Query<&mut GridCoords, With<player::Player>>
) {
    for player_transform in &player {
        let pos_x = player_transform.x;
        let pos_y = player_transform.y;

        // let pos = player_transform.translation;

        for mut transform in &mut camera {
            transform.translation.x = pos_x as f32 * 15.;
            transform.translation.y = pos_y as f32 * 15.;
        }
    }
}

// Tests for the abstract factory dialogue class
#[cfg(test)]
mod test{
    use array2d::Array2D;
    use dialogue_factory::*;
    use graph_system::*;
    use super::graph_system;
    use super::dice_system::*;
    use dice::dice::roll;
    use super::player::*;
    use super::enemy::*;

    #[test]
    fn test_init_player_entity_factory(){
        let dialogue_fact = DialogueFactory {};
        let dialogue_npc_factory = AbstractDialogueFactory::create_name_text_dialogue(&dialogue_fact); 
        
        let npc_name = dialogue_npc_factory.name_dialogue(String::from("Bobby"));
        let name = String::from("Bobby");

        assert_eq!(&npc_name, &name);
    }

    #[test]
    fn test_gen_matrix (){
        let mut graph_base = graph_system::AdjMatrixGraph{
            num_vertices: 0,
            directed: false,
            matrix: Array2D::filled_with(0, 0, 0)
        };

        graph_base.gen_empty_matrix(5);
        assert_eq!(graph_base.num_vertices, 5);
        assert_ne!(graph_base.matrix, Array2D::filled_with(0, 0, 0));
    }

    #[test]
    fn test_add_edge(){
        let mut graph_base = graph_system::AdjMatrixGraph{
            num_vertices: 5,
            directed: false,
            matrix: Array2D::filled_with(0, 0, 0),
        };

        graph_base.gen_empty_matrix(4);
        graph_base.add_edge(0, 1, 5);
        graph_base.add_edge(1, 2, 10);
        graph_base.add_edge(2, 3, 14);
        graph_base.add_edge(0, 2, 51);

        assert_eq!(graph_base.get_edge_weight(0, 1).unwrap_or(&0), Some(5).as_ref().unwrap_or(&0));
        assert_eq!(graph_base.get_adj_vertices(0), vec![1, 2]);
        assert_eq!(graph_base.get_adj_vertices(2), vec![0, 1, 3]);
        println!("what is edge weight? {:?}", graph_base.get_edge_weight(0, 1).unwrap_or(&0));
        println!("what are the adj vertices for 0? {:?}", graph_base.get_adj_vertices(0));
        println!("what are the adj vertices for 2? {:?}", graph_base.get_adj_vertices(2));

    }

    #[test]
    fn test_num_of_vertices() { 
        let mut graph_base = graph_system::AdjMatrixGraph{
            num_vertices: 4,
            directed: false,
            matrix: Array2D::filled_with(0, 0, 0),
        };

        graph_base.gen_empty_matrix(4);
        graph_base.add_edge(0, 1, 12);
        graph_base.add_edge(1, 2, 11);
        graph_base.add_edge(2, 3, 1);
        graph_base.add_edge(1, 3, 5);
        graph_base.add_edge(0, 2, 18);
        graph_base.display();

        assert_eq!(graph_base.num_of_vertices(), 4);
    }

    #[test]
    fn test_get_edge_weight() {
        let mut graph_base = graph_system::AdjMatrixGraph{
            num_vertices: 4,
            directed: false,
            matrix: Array2D::filled_with(0, 0, 0),
        };

        graph_base.gen_empty_matrix(4);
        graph_base.add_edge(0, 1, 12);
        graph_base.add_edge(1, 2, 11);
        graph_base.add_edge(2, 3, 1);
        graph_base.add_edge(1, 3, 5);
        graph_base.add_edge(0, 2, 18);

        assert_eq!(graph_base.get_edge_weight(1, 2), Some(&11));
        println!("{:?}", graph_base.get_edge_weight(1, 2).unwrap_or(&0));
    }

    #[test]
    fn test_roll_result_from_struct() {
        let dice = DiceEventSystem{
            dice_event: roll("1d20"),
        };

        let result = &dice.dice_event;

        assert_eq!(dice.dice_event.total, result.total);
        assert_eq!(dice.dice_event.total as u32, result.total as u32);
    }

    #[test]
    fn test_roll_for_event() {
        let event_dice = DiceEventSystem{
            dice_event: roll("1d20"),
        };

        let mut graph_base = graph_system::AdjMatrixGraph{
            num_vertices: 0,
            directed: false,
            matrix: Array2D::filled_with(0, 0, 0),
        };

        graph_base.gen_empty_matrix(4);
        graph_base.add_edge(0, 1, event_dice.dice_event.total as u32);
        graph_base.add_edge(1, 2, event_dice.dice_event.total as u32);
        graph_base.add_edge(2, 3, event_dice.dice_event.total as u32);
        graph_base.add_edge(1, 3, event_dice.dice_event.total as u32);
        graph_base.add_edge(0, 2, event_dice.dice_event.total as u32);

        assert_eq!(graph_base.get_edge_weight(0, 1), Some(&(event_dice.dice_event.total as u32)));
        assert_eq!(graph_base.get_edge_weight(1, 2), Some(&(event_dice.dice_event.total as u32)));
        assert_eq!(graph_base.get_edge_weight(2, 3), Some(&(event_dice.dice_event.total as u32)));
        assert_eq!(graph_base.get_edge_weight(1, 3), Some(&(event_dice.dice_event.total as u32)));
        assert_eq!(graph_base.get_edge_weight(0, 2), Some(&(event_dice.dice_event.total as u32)));
    
        assert_eq!(graph_base.get_adj_vertices(0), vec![1, 2]);
        assert_eq!(graph_base.get_adj_vertices(2), vec![0, 1, 3]);
        println!("what are the adj vertices for 0? {:?}", graph_base.get_adj_vertices(0));
        println!("what are the adj vertices for 2? {:?}", graph_base.get_adj_vertices(2));
    }

    #[test]
    fn test_basic_roll_for_attack() {
        let player_info = PlayerName{
            player_name: String::from("Ariel")
        };

        let mut player_health = PlayerHealth{
            player_hp: 25
        };

        let mut player_attack_dice = DiceAttackSystem{
            dice_attack: roll("1d20"),
        };

        let enemy_info = EnemyName{
            enemy_name: String::from("Mufaba")
        };

        let mut enemy_health = EnemyHealth{
            enemy_hp: 25
        };

        let mut enemy_attck_dice = DiceAttackSystem{
            dice_attack: roll("1d20"),
        };

        let attack_result = player_attack_dice.get_attack_roll().total;
        let enemy_attack_result = enemy_attck_dice.get_attack_roll().total;

        println!("what is the player attack result? {:?}", attack_result);
        println!("get the enemy attack result, {:?}", enemy_attack_result);

        // losing hp from attack dice roll
        let player_hp_attacked_result = player_health.player_hp as isize - enemy_attack_result;
        let enemy_hp_attacked_result = enemy_health.enemy_hp as isize - attack_result;

        player_health.player_hp = player_hp_attacked_result as u32;
        enemy_health.enemy_hp = enemy_hp_attacked_result as u32;

        println!("the player hp result: {:?}", player_hp_attacked_result); 
        println!("the enemy hp result: {:?}", enemy_hp_attacked_result); 

        assert_ne!(attack_result, 0);
        assert_ne!(enemy_attack_result, 0);
        assert_eq!(player_info.player_name, String::from("Ariel"));
        assert_eq!(enemy_info.enemy_name, String::from("Mufaba"));
        assert_eq!(player_health.player_hp as isize, player_hp_attacked_result);
        assert_eq!(enemy_health.enemy_hp as isize, enemy_hp_attacked_result);
    }

    #[test]
    fn test_basic_roll_for_agility() {
        let player_info = PlayerName{
            player_name: String::from("Ariel")
        };

        let mut player_health = PlayerHealth{
            player_hp: 25
        };

        let mut player_attack_dice = DiceAttackSystem{
            dice_attack: roll("1d20"),
        };

        let mut player_agility_dice = DiceAgilitySystem{
            dice_agility: roll("1d20"),
        };

        let enemy_info = EnemyName{
            enemy_name: String::from("Mufaba")
        };

        let mut enemy_health = EnemyHealth{
            enemy_hp: 25
        };

        let mut enemy_attck_dice = DiceAttackSystem{
            dice_attack: roll("1d20"),
        };

        let attack_result = player_attack_dice.get_attack_roll().total;
        let enemy_attack_result = enemy_attck_dice.get_attack_roll().total;

        println!("what is the player attack result? {:?}", attack_result);
        println!("get the enemy attack result, {:?}", enemy_attack_result);

        
            // losing hp from the attack dice roll
            let player_hp_attacked_result = player_health.player_hp as isize - enemy_attack_result;
            let enemy_hp_attacked_result = enemy_health.enemy_hp as isize - attack_result;

        player_health.player_hp = player_hp_attacked_result as u32;
        enemy_health.enemy_hp = enemy_hp_attacked_result as u32;

        println!("the player hp result: {:?}", player_hp_attacked_result); 
        println!("the enemy hp result: {:?}", enemy_hp_attacked_result); 

        assert_ne!(attack_result, 0);
        assert_ne!(enemy_attack_result, 0);
        assert_eq!(player_info.player_name, String::from("Ariel"));
        assert_eq!(enemy_info.enemy_name, String::from("Mufaba"));
        assert_eq!(player_health.player_hp as isize, player_hp_attacked_result);
        assert_eq!(enemy_health.enemy_hp as isize, enemy_hp_attacked_result);
    }

    #[test]
    fn test_set_and_get_player_name() {
        let player_info = PlayerName{
            player_name: String::from("Ariel")
        };

        let name = player_info.get_player_name();

        println!("{:?}", name);

        assert_eq!(name, String::from("Ariel"));

        let mut player_set_info = PlayerName{
            player_name: String::from("None")
        };

        player_set_info.set_player_name(String::from("Bob"));

        let get_name = player_set_info.get_player_name();
        
        println!("{:?}", get_name);
        assert_eq!(get_name, String::from("Bob"));
    }
}

