use bevy::prelude::*;
use bevy::window::*;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

mod debug;
mod menu_cursor;
mod player;
mod selection;
mod main_menu;

fn main () {
    App::new()
    .add_plugins((DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            fit_canvas_to_parent: true, 
            present_mode: PresentMode::AutoVsync, 
            mode: WindowMode::Fullscreen, 
            resolution: (1920., 1080.).into(), 
            title: "Terminal Overload".into(), 
            window_theme: Some(WindowTheme::Dark), 
            visible: true,
            ..Default::default()}),
            ..Default::default()
    }),  
    LogDiagnosticsPlugin::default(),
    FrameTimeDiagnosticsPlugin,))
    .add_plugins(main_menu::MainMenu)
    .add_plugins(selection::SelectionPlugin)
    .add_plugins(menu_cursor::MenuCursorPlugin)
    // .add_plugins(debug::DebugPlugin)
    .add_systems(Update, bevy::window::close_on_esc)
    .insert_resource(ClearColor(Color::BLACK))
    .run();
}