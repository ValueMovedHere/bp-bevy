use avian3d::prelude::*;
use bevy::{
    light::DirectionalLightShadowMap, //
    prelude::*,
};

mod controller;
mod initialise;
mod state;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(controller::FpsControllerPlugin)
        .add_systems(Startup, initialise::load_scene)
        .add_systems(Startup, initialise::set_scene_colliders)
        .add_systems(Startup, controller::setup_controller)
        .add_systems(Update, (controller::respawn, state::manage_cursor))
        .run();
}
