use bevy::prelude::*;

use crate::selection::Position;

const STARTING_CURSOR_TRANSLATION: Vec3 = Vec3::new(30., 8., 0.);

#[derive(Bundle)]
struct MenuCursorBundle{
    position: Position,
    cursor: SceneBundle,
}

pub struct MenuCursorPlugin;
impl Plugin for MenuCursorPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor);
    }
}

fn spawn_cursor(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(MenuCursorBundle{
        position: Position{
            x: 30.,
            y: 8., 
        },
        cursor: SceneBundle{
            scene: asset_server.load("right.png"),
            transform: Transform::from_translation(STARTING_CURSOR_TRANSLATION),
            ..default()
        }, 
    });
}
