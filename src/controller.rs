use ::std::f32::consts::TAU;

use avian3d::prelude::*;
use bevy::camera::Exposure;
use bevy::prelude::*;
pub use bevy_fps_controller::controller::*;

const SPAWN_POINT: Vec3 = Vec3::new(1.0, 0.5, 0.0);

pub fn setup_controller(mut commands: Commands) {
    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    let height = 1.0;
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
                sensitivity: 0.01,
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
        Exposure::BLENDER,
        RenderPlayer { logical_entity },
    ));
}

pub fn respawn(mut query: Query<(&mut Transform, &mut LinearVelocity)>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }

        velocity.0 = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}
