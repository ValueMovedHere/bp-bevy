use avian3d::prelude::*;
use bevy::{
    light::CascadeShadowConfigBuilder, //
    prelude::*,
};

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
    ));
    commands.spawn(WorldAssetRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("models/backrooms_with_baked_textures.glb"),
    )));
}

pub fn set_scene_colliders(mut commands: Commands) {
    let colliders_cuboid = Collider::compound(vec![
        (
            Vec3::new(0.0, 0.0, 0.0), // floor
            Quat::IDENTITY,
            Collider::cuboid(4.290, 0.001, 27.320),
        ),
        (
            Vec3::new(-1.148, 0.853, 14.883), // steps
            Quat::from_rotation_x(-26.89f32.to_radians()),
            Collider::cuboid(2.286, 0.001, 3.950),
        ),
        (
            Vec3::new(1.194, 2.550, 15.301), // steps
            Quat::from_rotation_x(25.91f32.to_radians()),
            Collider::cuboid(2.282, 0.001, 3.692),
        ),
        (
            // steps1
            Vec3::new(1.148, 0.853, -14.883),
            Quat::from_rotation_x(26.89f32.to_radians()),
            Collider::cuboid(2.286, 0.001, 3.950),
        ),
        (
            // steps1
            Vec3::new(-1.194, 2.550, -15.301),
            Quat::from_rotation_x(-25.91f32.to_radians()),
            Collider::cuboid(2.282, 0.001, 3.692),
        ),
        (
            // wall between steps
            Vec3::new(0.000, 1.664, 15.187),
            Quat::IDENTITY,
            Collider::cuboid(0.317, 3.283, 3.113),
        ),
        (
            // floor between steps
            Vec3::new(0.000, 1.569, 17.602),
            Quat::IDENTITY,
            Collider::cuboid(4.662, 0.284, 1.949),
        ),
        (
            // wall between steps
            Vec3::new(0.000, 1.659, -15.185),
            Quat::IDENTITY,
            Collider::cuboid(0.286, 3.320, 3.097),
        ),
        (
            // floor between steps
            Vec3::new(0.000, 1.569, -17.602),
            Quat::IDENTITY,
            Collider::cuboid(4.662, 0.284, 1.949),
        ),
        (
            // side wall
            Vec3::new(2.224, 1.704, 0.000),
            Quat::IDENTITY,
            Collider::cuboid(0.171, 3.314, 36.825),
        ),
        (
            // side wall
            Vec3::new(-2.224, 1.704, 0.000),
            Quat::IDENTITY,
            Collider::cuboid(0.171, 3.314, 36.825),
        ),
    ]);
    // let collider_side_wall = InfinitePlane3d::new(Vec3::new(1.000, 0.000, 0.000));
    let collider_end_wall = InfinitePlane3d::new(Vec3::new(0.000, 0.000, 1.000));
    commands.spawn((RigidBody::Static, colliders_cuboid));
    // commands.spawn((
    //     Transform::from_xyz(2.096, 0.000, 0.000),
    //     RigidBody::Static,
    //     Collider::from(collider_side_wall),
    // ));
    // commands.spawn((
    //     Transform::from_xyz(-2.204, 0.000, 0.000),
    //     RigidBody::Static,
    //     Collider::from(collider_side_wall),
    // ));
    commands.spawn((
        Transform::from_xyz(0.000, 0.000, 18.508),
        RigidBody::Static,
        Collider::from(collider_end_wall),
    ));
    commands.spawn((
        Transform::from_xyz(0.000, 0.000, -18.508),
        RigidBody::Static,
        Collider::from(collider_end_wall),
    ));
}
