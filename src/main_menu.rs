use crate::states::*;
use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },
    prelude::*,
};
use std::process;

pub struct MainMenu;
impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::TitleMenu), title_scene)
            .add_systems(Update, selector)
            .add_systems(OnExit(GameState::TitleMenu), despawn_screen::<OnMenuScreen>);
    }
}

#[derive(Component)]
enum Selection {
    Top,
    Middle,
    Bottom,
}

#[derive(Component)]
struct Options {
    top_sel: bool,
    mid_sel: bool,
    bot_sel: bool,
}

#[derive(Component)]
struct OnMenuScreen;

fn title_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/alphacorsa.personal-use.ttf"),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(),
        OnMenuScreen, // 3. Enable bloom for the camera
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("workers/right.png"),
            transform: Transform::from_xyz(-280., -150., 0.),
            ..default()
        },
        Selection::Top,
        Options {
            top_sel: true,
            mid_sel: false,
            bot_sel: false,
        },
        OnMenuScreen,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("|- TERMINAL OVERLORD -|", text_style.clone()),
            transform: Transform::from_xyz(5., 100., 0.),
            ..default()
        },
        OnMenuScreen,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("New Game", text_style.clone()),
            transform: Transform::from_xyz(-20., -150., 0.),
            ..default()
        },
        OnMenuScreen,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Load Save", text_style.clone()),
            transform: Transform::from_xyz(-20., -250., 0.),
            ..default()
        },
        OnMenuScreen,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Exit", text_style.clone()),
            transform: Transform::from_xyz(-148., -350., 0.),
            ..default()
        },
        OnMenuScreen,
    ));
}

fn selector(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut sprite_position: Query<(&mut Options, &mut Selection, &mut Transform)>,
) {
    for (mut opt, mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Selection::Top => {
                transform.translation = Vec3 {
                    x: -280.,
                    y: -150.,
                    z: 0.,
                }
            }
            Selection::Middle => {
                transform.translation = Vec3 {
                    x: -280.,
                    y: -250.,
                    z: 0.,
                }
            }
            Selection::Bottom => {
                transform.translation = Vec3 {
                    x: -280.,
                    y: -350.,
                    z: 0.,
                }
            }
        }

        if (keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp)) && !opt.top_sel {
            *logo = Selection::Top;
            opt.top_sel = true;
            opt.mid_sel = false;
            opt.bot_sel = false;
        } else if (keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown))
            && !opt.mid_sel
            && !opt.bot_sel
        {
            *logo = Selection::Middle;
            opt.mid_sel = true;
            opt.top_sel = false;
            opt.bot_sel = false;
        } else if (keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown))
            && !opt.bot_sel
        {
            *logo = Selection::Bottom;
            opt.bot_sel = true;
            opt.mid_sel = false;
            opt.top_sel = false;
        }

        if opt.top_sel && keys.just_pressed(KeyCode::Enter) {
            info!("the enter key was pressed for new game!");
            game_state.set(GameState::Running);
        }

        if opt.mid_sel && keys.just_pressed(KeyCode::Enter) {
            info!("the enter key was pressed for load game!");
        }

        if opt.bot_sel && keys.just_pressed(KeyCode::Enter) {
            info!("the enter key was pressed for exit game!");
            process::exit(1);
        }
    }
}
