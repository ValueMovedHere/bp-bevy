use avian3d::prelude::*;
use bevy::{
    light::DirectionalLightShadowMap, //
    prelude::*,
};
use bevy_easy_gif::GifPlugin;
use bevy_inspector_egui::{self, bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod controller;
mod initialise;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(controller::FpsControllerPlugin)
        .add_plugins(GifPlugin)
        .add_systems(Startup, initialise::load_scene)
        .add_systems(Startup, initialise::set_scene_colliders)
        .add_systems(Startup, controller::setup_controller)
        .add_systems(Startup, initialise::load_gif)
        .add_systems(Update, controller::respawn)
        .run();
}
