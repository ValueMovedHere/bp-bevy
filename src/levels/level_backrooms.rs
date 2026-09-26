use avian3d::prelude::*;
use bevy::prelude::*;

use serde_scene::audio;
use serde_scene::collider;

use crate::Level;
use crate::controller::INITIAL_VELOCITY;

pub const SPAWN_POINT: Vec3 = Vec3::new(0f32, 0f32, 0f32);

#[derive(Component)]
pub struct LevelBackroomsBakedRes;

pub fn setup_colliders(mut commands: Commands) {
    let colliders_vec =
        collider::from_json("./data/levels/level-backrooms-baked/colliders/colliders.json");
    let scene_collider = Collider::compound(colliders_vec);
    commands.spawn((RigidBody::Static, scene_collider, LevelBackroomsBakedRes));
}

pub fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load glb scene
    commands.spawn((
        LevelBackroomsBakedRes,
        WorldAssetRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(
                "models/levels/level-backrooms-baked/backrooms_with_baked_textures.glb",
            ),
        )),
    ));
    // sound effect of the lights
    let playback_settings = PlaybackSettings::LOOP.with_spatial(true);
    let audio_handler = asset_server.load("audios/levels/level-backrooms-baked/lights.mp3");
    let audio_player = AudioPlayer::new(audio_handler);
    let lights_audio_entities_vec = audio::from_json(
        "data/levels/level-backrooms-baked/audios/lights.json",
        audio_player,
        playback_settings,
    );
    commands.spawn_batch(lights_audio_entities_vec);
}

pub fn respawn(query: Query<(&mut Transform, &mut LinearVelocity)>) {
    for (mut transform, mut velocity) in query {
        // 位置
        transform.translation = SPAWN_POINT;
        // 速度归零
        velocity.0 = INITIAL_VELOCITY;
    }
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<LevelBackroomsBakedRes>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

// 让我们假设在游戏中触发了切到下一级的逻辑
pub fn next_scene(mut next_state: ResMut<NextState<Level>>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::KeyN) {
        next_state.set(Level::LevelAbandonedvrgallery);
    }
}
