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
    let collider_end_wall = InfinitePlane3d::new(Vec3::new(0.000, 0.000, 1.000));
    commands.spawn((RigidBody::Static, colliders_cuboid, LevelSchoolRes));
    commands.spawn((
        Transform::from_xyz(0.000, 0.000, 18.508),
        RigidBody::Static,
        Collider::from(collider_end_wall),
        LevelSchoolRes,
    ));
    commands.spawn((
        Transform::from_xyz(0.000, 0.000, -18.508),
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

pub fn print_state(state: Res<State<crate::state::Level>>) {
    println!("Current state: {:#?}", state);
}
