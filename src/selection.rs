use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_cursor_position);
    }
}

fn update_cursor_position(mut query: Query<&mut Transform>, time: Res<Time>) {
    //for mut position in query.iter_mut(){
    // position.translation.x = 1.;
    // position.translation.y = 1.;
    //}
}
