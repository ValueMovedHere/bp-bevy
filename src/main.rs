use avian3d::prelude::*;
use bevy::{
    light::DirectionalLightShadowMap, //
    prelude::*,
};

mod controller;
mod initialise;
mod liminal_space_scene;
mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<state::Level>()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        // .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(controller::FpsControllerPlugin)
        .add_systems(OnEnter(state::Level::School), initialise::load_scene)
        .add_systems(
            OnEnter(state::Level::School),
            initialise::set_scene_colliders,
        )
        .add_systems(Startup, controller::setup_controller)
        .add_systems(Update, (controller::respawn, state::manage_cursor))
        .run();
}
