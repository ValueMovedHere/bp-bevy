use avian3d::prelude::*;
use bevy::prelude::*;

mod controller;
mod levels;
mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<state::Level>()
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(controller::FpsControllerPlugin)
        .add_systems(
            OnEnter(state::Level::LevelSchool),
            levels::level_school::load_scene,
        )
        .add_systems(
            OnEnter(state::Level::LevelSchool),
            (
                levels::level_school::set_scene_colliders,
                levels::level_school::set_cursor,
            ),
        )
        .add_systems(Startup, controller::setup_controller)
        .add_systems(
            Update,
            levels::level_school::next_level.run_if(in_state(state::Level::LevelSchool)),
        )
        .add_systems(
            OnExit(state::Level::LevelSchool),
            (
                levels::level_school::cleanup,
                levels::level_school::next_level,
            ),
        )
        .add_systems(
            OnEnter(state::Level::LevelBackroomsBaked),
            (
                levels::level_backrooms::setup_colliders,
                levels::level_backrooms::load_scene,
                controller::respawn,
            )
                .chain(),
        )
        .add_systems(Update, (state::manage_cursor))
        .run();
}
