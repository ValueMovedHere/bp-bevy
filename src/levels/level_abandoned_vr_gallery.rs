use avian3d::prelude::*;
use bevy::prelude::*;
use serde_scene::from_json;

use crate::controller::INITIAL_VELOCITY;

pub const SPAWN_POINT: Vec3 = Vec3::new(0f32, 0f32, 0f32);

#[derive(Component)]
pub struct LevelAbandonedVrGallery;

pub fn load_scene() {}

pub fn setup_colliders() {}

pub fn cleanup() {}
