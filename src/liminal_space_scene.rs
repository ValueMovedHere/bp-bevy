use avian3d::prelude::*;
use bevy::prelude::*;

const H: f32 = 2.5;
const Y: f32 = 1.25;

pub fn cleanup_school_scene(mut commands: Commands, assets: Res<Assets<WorldAsset>>) {}

pub fn collider_liminal_space(mut commands: Commands) {
    let colliders = Collider::compound(vec![
        (
            Vec3::new(0f32, Y, 0f32),
            Quat::IDENTITY,
            Collider::cuboid(10f32, H, 10f32),
        ),
        (
            Vec3::new(-2.779f32, Y, 2.777f32),
            Quat::IDENTITY,
            Collider::cuboid(0.244f32, H, 1.758f32),
        ),
        (
            Vec3::new(2.195f32, Y, 1.002f32),
            Quat::IDENTITY,
            Collider::cuboid(0.207f32, H, 8.123f32),
        ),
        (
            Vec3::new(-2.891f32, Y, 1.866),
            Quat::IDENTITY,
            Collider::cuboid(4.456f32, H, 0.235f32),
        ),
        (
            Vec3::new(-2.281f32, Y, -2.831f32),
            Quat::IDENTITY,
            Collider::cuboid(0.207f32, H, 4.485f32),
        ),
        (
            Vec3::new(3.030f32, Y, -5.071f32),
            Quat::IDENTITY,
            Collider::cuboid(0.582f32, H, 0.640f32),
        ),
        (
            Vec3::new(5.003f32, Y, -3.230f32),
            Quat::IDENTITY,
            Collider::cuboid(0.582f32, H, 0.640f32),
        ),
        (
            Vec3::new(3.386f32, Y, 4.968f32),
            Quat::IDENTITY,
            Collider::cuboid(0.582f32, H, 0.640f32),
        ),
    ]);
    let collider1 = Collider::cuboid(10f32, H, 10f32);
    commands.spawn((RigidBody::Static, colliders));
    commands.spawn((
        Transform::from_xyz(0f32, -1.25, 0f32),
        RigidBody::Static,
        collider1,
    ));
}
