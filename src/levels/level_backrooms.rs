use avian3d::prelude::*;
use bevy::prelude::*;
use serde_scene::from_json;

pub fn setup_colliders(mut commands: Commands) {
    let colliders = from_json("./data/backrooms_colliders.json");
    let scene_collider = Collider::compound(colliders);
    commands.spawn((RigidBody::Static, scene_collider));
}

pub fn cleanup() {}
