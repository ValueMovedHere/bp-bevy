use avian3d::prelude::*;
use bevy::prelude::*;
use serde_scene::from_json;

use crate::controller::INITIAL_VELOCITY;

pub const SPAWN_POINT: Vec3 = Vec3::new(0f32, 1.1f32, 0f32);

#[derive(Component)]
pub struct LevelBackroomsBakedRes;

pub fn setup_colliders(mut commands: Commands) {
    let colliders_vec = from_json("./data/levels/level-backrooms-baked/colliders/colliders.json");
    let scene_collider = Collider::compound(colliders_vec);
    commands.spawn((RigidBody::Static, scene_collider));
}

pub fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        LevelBackroomsBakedRes,
        WorldAssetRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(
                "models/levels/level-backrooms-baked/backrooms_with_baked_textures.glb",
            ),
        )),
    ));
}

pub fn respawn(query: Query<(&mut Transform, &mut LinearVelocity)>) {
    for (mut transform, mut velocity) in query {
        // 位置
        transform.translation = SPAWN_POINT;
        // 速度归零
        velocity.0 = Vec3::new(0f32, 0f32, 0f32);
    }
}

pub fn cleanup() {}
