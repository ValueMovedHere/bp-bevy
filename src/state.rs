use bevy::prelude::*;
use bevy::window::{
    CursorGrabMode, //
    CursorOptions,
};
use bevy_fps_controller::controller::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default, States)]
pub enum Level {
    #[default]
    LevelSchool,
    LevelBackroomsBaked,
}

pub fn manage_cursor(
    key: Res<ButtonInput<KeyCode>>,
    mut cursor: Single<&mut CursorOptions>,
    mut controller_query: Query<&mut FpsController>,
) {
    if key.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    } else if key.just_pressed(KeyCode::KeyC) {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
}
