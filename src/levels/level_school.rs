use avian3d::prelude::*;
use bevy::{
    light::CascadeShadowConfigBuilder, //
    prelude::*,
    window::CursorOptions,
};

use crate::state::Level;

#[derive(Component)]
pub struct LevelSchoolRes;

pub fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            illuminance: light_consts::lux::HALLWAY,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .build(),
        LevelSchoolRes,
    ));
    commands.spawn((
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/school_bp.glb")),
        ),
        LevelSchoolRes,
    ));
}

pub fn set_scene_colliders(mut commands: Commands) {
    let colliders_cuboid = Collider::compound(vec![
        (
            Vec3::new(0f32, 0f32, 0f32), // floor
            Quat::IDENTITY,
            Collider::cuboid(4.29f32, 0.001f32, 27.32f32),
        ),
        (
            Vec3::new(-1.148f32, 0.853f32, 14.883f32), // steps
            Quat::from_rotation_x(-26.89f32.to_radians()),
            Collider::cuboid(2.286f32, 0.001f32, 3.95f32),
        ),
        (
            Vec3::new(1.194f32, 2.55f32, 15.301f32), // steps
            Quat::from_rotation_x(25.91f32.to_radians()),
            Collider::cuboid(2.282f32, 0.001f32, 3.692f32),
        ),
        (
            // steps1
            Vec3::new(1.148f32, 0.853f32, -14.883f32),
            Quat::from_rotation_x(26.89f32.to_radians()),
            Collider::cuboid(2.286f32, 0.001f32, 3.95f32),
        ),
        (
            // steps1
            Vec3::new(-1.194f32, 2.55f32, -15.301f32),
            Quat::from_rotation_x(-25.91f32.to_radians()),
            Collider::cuboid(2.282f32, 0.001f32, 3.692f32),
        ),
        (
            // wall between steps
            Vec3::new(0f32, 1.664f32, 15.187f32),
            Quat::IDENTITY,
            Collider::cuboid(0.317f32, 3.283f32, 3.113f32),
        ),
        (
            // floor between steps
            Vec3::new(0f32, 1.569f32, 17.602f32),
            Quat::IDENTITY,
            Collider::cuboid(4.662f32, 0.284f32, 1.949f32),
        ),
        (
            // wall between steps
            Vec3::new(0f32, 1.659f32, -15.185f32),
            Quat::IDENTITY,
            Collider::cuboid(0.286f32, 3.32f32, 3.097f32),
        ),
        (
            // floor between steps
            Vec3::new(0f32, 1.569f32, -17.602f32),
            Quat::IDENTITY,
            Collider::cuboid(4.662f32, 0.284f32, 1.949f32),
        ),
        (
            // side wall
            Vec3::new(2.224f32, 1.704f32, 0f32),
            Quat::IDENTITY,
            Collider::cuboid(0.171f32, 3.314f32, 36.825f32),
        ),
        (
            // side wall
            Vec3::new(-2.224f32, 1.704f32, 0f32),
            Quat::IDENTITY,
            Collider::cuboid(0.171f32, 3.314f32, 36.825f32),
        ),
    ]);
    let collider_end_wall = InfinitePlane3d::new(Vec3::new(0f32, 0f32, 1f32));
    commands.spawn((RigidBody::Static, colliders_cuboid, LevelSchoolRes));
    commands.spawn((
        Transform::from_xyz(0f32, 0f32, 18.508f32),
        RigidBody::Static,
        Collider::from(collider_end_wall),
        LevelSchoolRes,
    ));
    commands.spawn((
        Transform::from_xyz(0f32, 0f32, -18.508f32),
        RigidBody::Static,
        Collider::from(collider_end_wall),
        LevelSchoolRes,
    ));
}

pub fn set_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
    cursor.visible = false;
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<LevelSchoolRes>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn next_level(
    mut next_state: ResMut<NextState<Level>>,
    query: Query<&Transform, With<LinearVelocity>>,
) {
    for player in query {
        if player.translation.y < -10f32 {
            next_state.set(Level::LevelBackroomsBaked);
        }
    }
}
