use bevy::prelude::*;
use bevy::window::*;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_ascii_terminal::{prelude::*, TiledCameraBundle};

mod debug;
mod menu_cursor;
mod player;
mod selection;

#[derive(Component, Debug)]
pub struct GameTerminal;

pub const VIEWPORT_SIZE: [u32;2] = [80,40];
pub const UI_SIZE: [u32;2] = [VIEWPORT_SIZE[0],8];
pub const GAME_SIZE: [u32;2] = [VIEWPORT_SIZE[0], VIEWPORT_SIZE[1] - UI_SIZE[1]];

fn title_scene(mut commands: Commands) {
    // Create the terminal
    let term_y = VIEWPORT_SIZE[1] as u32 / 2 - GAME_SIZE[1] as u32 / 2; 
    let term = Terminal::new([20, term_y]).with_border(Border::single_line());
 
    let mut term_bundle = TerminalBundle::from(term).with_size([GAME_SIZE[0], GAME_SIZE[1] + 2]);
    term_bundle.terminal.put_string([36, 10], "New  Game".fg(Color::WHITE));
    term_bundle.terminal.put_string([35, 8], " Load Save".fg(Color::WHITE));
    term_bundle.terminal.put_string([36, 6], "Exit".fg(Color::WHITE));

    commands.spawn(term_bundle);

    let mut terminal = Terminal::new([20,3]).with_border(Border::single_line());
    // Draw title to the terminal
    terminal.put_string([1, 1], "Terminal Overload".fg(Color::WHITE));

    commands.spawn(
        // Spawn the terminal bundle from our terminal
        TerminalBundle::from(terminal));

    let totalx = GAME_SIZE[0];
    let totaly = GAME_SIZE[1] + UI_SIZE[1];

    commands.spawn(TiledCameraBundle::new().with_tile_count([totalx, totaly]));
    
}

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
    .add_plugins(TerminalPlugin)
    .add_plugins(selection::SelectionPlugin)
    .add_plugins(menu_cursor::MenuCursorPlugin)
    .add_plugins(debug::DebugPlugin)
    .add_systems(Startup, title_scene)
    .add_systems(Update, bevy::window::close_on_esc)
    .insert_resource(ClearColor(Color::BLACK))
    .run();
}