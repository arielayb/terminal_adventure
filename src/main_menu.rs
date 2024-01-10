use bevy::prelude::*;
use bevy_ascii_terminal::{prelude::*, TiledCameraBundle};

pub const VIEWPORT_SIZE: [u32;2] = [80,40];
pub const UI_SIZE: [u32;2] = [VIEWPORT_SIZE[0],8];
pub const GAME_SIZE: [u32;2] = [VIEWPORT_SIZE[0], VIEWPORT_SIZE[1] - UI_SIZE[1]];

pub struct MainMenu;
impl Plugin for MainMenu{
    fn build(&self, app: &mut App) {
        app
        .add_plugins(TerminalPlugin)
        .add_systems(Startup, title_scene)
        .add_systems(Update, selector);
    }
}

struct Cursor {
    pub glyph: char,
    pub fg_color: Color,
    pub bg_color: Color,
}

fn title_scene(mut commands: Commands) {
    // Create the terminal
    let term_y = VIEWPORT_SIZE[1] as u32 / 2 - GAME_SIZE[1] as u32 / 2; 
    let term = Terminal::new([20, term_y]).with_border(Border::single_line());
 
    let mut term_bundle = TerminalBundle::from(term).with_size([GAME_SIZE[0], GAME_SIZE[1] + 2]);

    term_bundle.terminal.put_tile([33, 10], Tile { glyph: '>', fg_color: Color::WHITE, bg_color: Color::BLACK, });
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

fn selector(keys: Res<Input<KeyCode>>, mut q: Query<&mut Terminal>) {

    let mut term = match q.get_single_mut() {
        Ok(term) => term,
        Err(_) => return,
    };

    if keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Down){
        term.clear();

        term.put_tile([33, 8], Tile { glyph: '>', fg_color: Color::WHITE, bg_color: Color::BLACK, });
    }


}