use std::f32::consts::TAU;

use avian3d::prelude::*;
use bevy::{
    camera::Exposure,
    light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use bevy_fps_controller::controller::*;

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.625, 0.0);

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(FpsControllerPlugin)
        .add_systems(Startup, load_scene)
        .add_systems(Startup, set_scene_colliders)
        .add_systems(Startup, setup_player)
        .add_systems(Update, respawn)
        .run();
}

fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .build(),
    ));
    commands.spawn(WorldAssetRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/school_bp.glb")),
    ));
}

fn setup_player(mut commands: Commands) {
    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    let height = 3.0;
    let logical_entity = commands
        .spawn((
            Collider::cylinder(0.5, height),
            // A capsule can be used but is NOT recommended
            // If you use it, you have to make sure each segment point is
            // equidistant from the translation of the player transform
            // Collider::capsule(0.5, height),
            Friction {
                dynamic_coefficient: 0.0,
                static_coefficient: 0.0,
                combine_rule: CoefficientCombine::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombine::Min,
            },
            LinearVelocity::ZERO,
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Mass(1.0),
            GravityScale(0.0),
            Transform::from_translation(SPAWN_POINT),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                air_acceleration: 80.0,
                ..default()
            },
        ))
        .insert(CameraConfig {
            height_offset: -0.5,
        })
        .id();

    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: TAU / 5.0,
            ..default()
        }),
        Exposure::SUNLIGHT,
        RenderPlayer { logical_entity },
    ));
}

fn respawn(mut query: Query<(&mut Transform, &mut LinearVelocity)>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }

        velocity.0 = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}

fn set_scene_colliders(mut commands: Commands) {
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
