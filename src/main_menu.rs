use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::states::GameState;

pub struct MainMenu;
impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::TitleMenu), title_scene)
            .add_systems(Update, selector);
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

fn title_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/alphacorsa.personal-use.ttf");
    let text_style = TextStyle {
        font: font.clone(),
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
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("right.png"),
            transform: Transform::from_xyz(-280., -150., 0.),
            ..default()
        },
        Selection::Top,
        Options {
            top_sel: false,
            mid_sel: false,
            bot_sel: false,
        },
    ));

    commands.spawn(Text2dBundle {
        text: Text::from_section("|- TERMINAL OVERLORD -|", text_style.clone()),
        transform: Transform::from_xyz(5., 100., 0.),
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_section("New Game", text_style.clone()),
        transform: Transform::from_xyz(-20., -150., 0.),
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_section("Load Save", text_style.clone()),
        transform: Transform::from_xyz(-20., -250., 0.),
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_section("Exit", text_style.clone()),
        transform: Transform::from_xyz(-148., -350., 0.),
        ..default()
    });
}

fn selector(
    keys: Res<Input<KeyCode>>,
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

        if (keys.just_pressed(KeyCode::W) || keys.just_pressed(KeyCode::Up)) && !opt.top_sel {
            *logo = Selection::Top;
            opt.top_sel = true;
            opt.mid_sel = false;
            opt.bot_sel = false;
        } else if (keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Down))
            && !opt.mid_sel
        {
            *logo = Selection::Middle;
            opt.mid_sel = true;
            opt.top_sel = false;
            opt.bot_sel = false;
        } else if (keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Down))
            && !opt.bot_sel
        {
            *logo = Selection::Bottom;
            opt.bot_sel = true;
            opt.mid_sel = true;
            opt.top_sel = false;
        }

        //assert_nq!(*logo, null);
    }
}
