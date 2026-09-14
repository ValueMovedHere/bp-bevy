use avian3d::prelude::*;
use bevy::prelude::*;
use serde_scene::from_json;

use crate::controller::INITIAL_VELOCITY;

pub const SPAWN_POINT: Vec3 = Vec3::new(0f32, 0f32, 0f32);

#[derive(Component)]
pub struct LevelAbandonedVrGalleryRes;

pub fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        LevelAbandonedVrGalleryRes,
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(
            "models/levels/level-abandoned-vr-gallery/level-abandoned-vr-gallery.glb",
        ))),
    ));
}

pub fn setup_colliders(mut commands: Commands) {
    let colliders_vec = from_json("./data/levels/level-abandoned-vr-gallery/colliders.json");
    let scene_collider = Collider::compound(colliders_vec);
    commands.spawn((
        LevelAbandonedVrGalleryRes,
        RigidBody::Static,
        scene_collider,
    ));
}

pub fn cleanup() {}
