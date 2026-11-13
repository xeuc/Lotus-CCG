use std::f32::consts::PI;

use bevy::prelude::*;
use crate::{GameState, pack_showcase::components::*};



// =============================================================================
// SCENE SETUP
// =============================================================================

pub fn reset_resources(
    mut progress:   ResMut<CardProgress>,
    mut next_batch: ResMut<NextBatchId>,
) {
    *progress   = CardProgress::default();
    *next_batch = NextBatchId::default();
}



pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::PackShowcase),
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::FRAC_PI_4,
            ..default()
        }),
        Transform::default()
            .with_translation(Vec3::new(-11.97946834564209, 30.476943969726563, 4.821719646453857))
            .with_rotation(Quat::from_xyzw(0.0, 0.7071067094802856, 0.0, 0.7071067094802856))
            .with_scale(Vec3::new(1.0, 1.0, 1.0)),
    ));
}


pub fn spawn_light(mut commands: Commands) {
    // Spawn light
    commands.spawn((
        DespawnOnExit(GameState::PackShowcase),
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(0.0, 35.0, 0.0),
    ));
}



/// Spawns Pack with StepIntro already in the bundle.
/// When this command flushes (together with all other OnEnter commands),
/// OnAdd<StepIntro> fires and finds Lid + Cards already in the world.
pub fn spawn_pack_body(
    mut commands:  Commands,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials_standard: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Pack,
        StepIntro,
        PendingTweens::default(),
        CurrentBatchId::default(),
        DespawnOnExit(GameState::PackShowcase),
        Mesh3d(meshes.add(Cuboid::new(140.0, 200.0, 0.1))),
        MeshMaterial3d(materials_standard.add(StandardMaterial {
            unlit: true,
            base_color: Color::srgb(0.10, 0.28, 0.72),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
        .with_rotation(
            Quat::from_rotation_y(PI/2.0)
        )
        .with_scale(Vec3::splat(0.3))
        ,

    ));

}


pub fn spawn_scene(
    mut commands:  Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        DespawnOnExit(GameState::PackShowcase),
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("models/Scene.gltf"),
        ))
    ));
}

