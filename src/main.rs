use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::*;

mod debug;
mod main_menu;
mod states;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    present_mode: PresentMode::AutoVsync,
                    mode: WindowMode::Windowed,
                    resolution: (1920., 1080.).into(),
                    title: "Terminal Overlord".into(),
                    window_theme: Some(WindowTheme::Dark),
                    visible: true,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_state::<states::GameState>()
        .add_state::<states::PausedState>()
        .add_plugins(main_menu::MainMenu)
        .add_plugins(world::World)
        // .add_plugins(debug::DebugPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
