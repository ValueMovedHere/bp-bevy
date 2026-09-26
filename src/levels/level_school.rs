use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{
    input::gestures::RotationGesture, light::CascadeShadowConfigBuilder, mesh::PlaneMeshBuilder,
    prelude::*, window::CursorOptions,
};

use bevy_easy_gif::Gif3d;
use serde_scene;

use crate::state::Level;

pub const SPAWN_POINT: Vec3 = Vec3::new(0f32, 0f32, 0f32);
const HEIGHT_OFFSET: f32 = 3.350f32;

#[derive(Component)]
pub struct LevelSchoolRes;

pub fn load_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .build(),
        LevelSchoolRes,
    ));
    // load the scene glb file
    let handle: Handle<WorldAsset> = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset("models/levels/level-school/school_bp.glb"));

    // let handle_tv_model = asset_server.load("assets/models/levels/level-school/television.glb");
    commands.spawn_batch([
        (
            WorldAssetRoot(handle.clone()),
            Transform::default(),
            LevelSchoolRes,
        ),
        (
            WorldAssetRoot(handle),
            Transform::from_xyz(0f32, HEIGHT_OFFSET, 0f32),
            LevelSchoolRes,
        ),
        // (
        //     WorldAssetRoot(handle_tv_model),
        //     Transform::from_translation(Vec3::new(0.748f32, -0.202 + HEIGHT_OFFSET, -12.493)),
        //     // .with_rotation(Quat::from_rotation_y(PI / 2f32)),
        //     LevelSchoolRes,
        // ),
    ]);
    let gif_handle = asset_server.load("images/levels/level-school/wtf.gif");
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            Vec3::new(1f32, 0f32, 0f32),
            Vec2::new(1.6f32, 0.5f32),
        ))),
        MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(0u8, 0u8, 0u8)).into()),
        Gif3d { handle: gif_handle },
    ));
}

pub fn set_scene_colliders(mut commands: Commands) {
    let mut colliders_cuboid_vec0 = vec![
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
    ];
    let mut colliders_cuboid_vec1 =
        serde_scene::collider::from_json("./data/levels/level-school/colliders/colliders.json");
    colliders_cuboid_vec0.append(&mut colliders_cuboid_vec1);
    // 阻止玩家从上方出去
    let mut colliders_vec2 = vec![
        (
            Vec3::new(1.296f32, 1.626f32 + HEIGHT_OFFSET, -13.922f32),
            Quat::IDENTITY,
            Collider::cuboid(2.430f32, 3.194f32, 0.543f32),
        ),
        (
            Vec3::new(-1.303f32, 1.626f32 + HEIGHT_OFFSET, 13.903f32),
            Quat::IDENTITY,
            Collider::cuboid(2.430f32, 3.194f32, 0.543f32),
        ),
    ];
    colliders_cuboid_vec0.append(&mut colliders_vec2);
    let colliders_cuboid = Collider::compound(colliders_cuboid_vec0);
    let collider_end_wall = InfinitePlane3d::new(Vec3::new(0f32, 0f32, 1f32));
    commands.spawn((RigidBody::Static, colliders_cuboid.clone(), LevelSchoolRes));
    commands.spawn((
        RigidBody::Static,
        colliders_cuboid,
        Transform::from_xyz(0f32, HEIGHT_OFFSET, 0f32),
        LevelSchoolRes,
    ));
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
        if player.translation.y < -0.5f32 {
            next_state.set(Level::LevelBackroomsBaked);
        }
    }
}
