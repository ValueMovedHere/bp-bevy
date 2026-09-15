use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_easy_gif::*;

mod controller;
mod levels;
mod state;

use state::{Level, manage_cursor};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<Level>()
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(GifPlugin)
        .add_plugins(controller::FpsControllerPlugin)
        .add_systems(
            OnEnter(Level::LevelSchool),
            levels::level_school::load_scene,
        )
        .add_systems(
            OnEnter(Level::LevelSchool),
            (
                levels::level_school::set_scene_colliders,
                levels::level_school::set_cursor,
            ),
        )
        .add_systems(Startup, controller::setup_controller)
        .add_systems(
            Update,
            (levels::level_school::next_level.run_if(in_state(Level::LevelSchool)),),
        )
        .add_systems(
            OnExit(Level::LevelSchool),
            (
                levels::level_school::cleanup,
                levels::level_school::next_level,
            ),
        )
        .add_systems(
            OnEnter(Level::LevelBackroomsBaked),
            (
                levels::level_backrooms::setup_colliders,
                levels::level_backrooms::load_scene,
                levels::level_backrooms::respawn,
            )
                .chain(),
        )
        // 如果处于该层级, 检查是否按下按键 N 并切换层级
        .add_systems(
            Update,
            levels::level_backrooms::next_scene.run_if(in_state(Level::LevelBackroomsBaked)),
        )
        // 退出这一层级时清理资源
        .add_systems(
            OnExit(Level::LevelBackroomsBaked),
            levels::level_backrooms::cleanup,
        )
        .add_systems(
            OnEnter(Level::LevelAbandonedvrgallery),
            (
                levels::level_abandoned_vr_gallery::load_scene,
                levels::level_abandoned_vr_gallery::setup_colliders,
                levels::level_abandoned_vr_gallery::respawn,
            )
                .chain(),
        )
        .add_systems(Update, manage_cursor)
        .run();
}
