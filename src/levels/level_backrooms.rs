use avian3d::prelude::*;
use bevy::prelude::*;
use serde_scene::collider::from_json;
use serde_scene::collider::sensor;

use crate::Level;
use crate::controller::INITIAL_VELOCITY;
use crate::controller::Player;

pub const SPAWN_POINT: Vec3 = Vec3::new(0f32, 0f32, 0f32);

#[derive(Component)]
pub struct LevelBackroomsBakedRes;

pub fn setup_colliders(mut commands: Commands) {
    let colliders_vec = from_json("./data/levels/level-backrooms-baked/colliders/colliders.json");
    let scene_collider = Collider::compound(colliders_vec);
    let room_sensor =
        sensor::from_json("./data/levels/level-backrooms-baked/colliders/sensor_room.json");
    commands.spawn((RigidBody::Static, scene_collider, LevelBackroomsBakedRes));
    commands.spawn(room_sensor);
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
        velocity.0 = INITIAL_VELOCITY;
    }
}

pub fn check_collision(
    _collision_event: On<CollisionStart>,
    mut enter_count: Local<u8>,
    _query: Query<&Player>,
    mut next_state: ResMut<NextState<Level>>,
) {
    *enter_count += 1;
    // 当玩家第二次进入房间的时候切到下一个场景
    if *enter_count >= 2 {
        next_state.set(Level::LevelAbandonedvrgallery);
    }
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<LevelBackroomsBakedRes>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

// // 让我们假设在游戏中触发了切到下一级的逻辑
// pub fn next_scene(mut next_state: ResMut<NextState<Level>>, key: Res<ButtonInput<KeyCode>>) {
//     if key.just_pressed(KeyCode::KeyN) {
//         next_state.set(Level::LevelAbandonedvrgallery);
//     }
// }
