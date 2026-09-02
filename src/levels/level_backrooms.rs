use avian3d::prelude::*;
use bevy::prelude::*;
use serde_scene::from_json;

#[derive(Component)]
pub struct LevelBackroomsBakedRes;

pub fn setup_colliders(mut commands: Commands) {
    let colliders_vec =
        from_json("./data/levels/level-backrooms-baked/colliders/backrooms_colliders.json");
    let scene_collider = Collider::compound(colliders_vec);
    commands.spawn((RigidBody::Static, scene_collider));
}

pub fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        LevelBackroomsBakedRes,
        WorldAssetRoot(
            asset_server.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("models/levels/backrooms-baked/backrooms_with_baked_textures.glb"),
            ),
        ),
    ));
}

pub fn cleanup() {}
