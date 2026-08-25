use bevy::{
    light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, load_scene)
        .run();
}

fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.7, 10.7, 11.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
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
