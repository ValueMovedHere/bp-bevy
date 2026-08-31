use avian3d::prelude::*;
use bevy::{
    light::DirectionalLightShadowMap, //
    prelude::*,
};

mod controller;
mod levels;
mod state;

fn main() {
    App::new()
        .init_state::<state::Level>()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(controller::FpsControllerPlugin)
        .add_systems(Startup, levels::level_school::load_scene)
        .add_systems(Startup, levels::level_school::set_scene_colliders)
        .add_systems(Startup, levels::level_school::set_cursor)
        .add_systems(Startup, controller::setup_controller)
        .add_systems(Update, (controller::respawn, state::manage_cursor))
        .run();
}
