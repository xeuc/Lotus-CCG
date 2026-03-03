// From https://bevy.org/examples/gltf/load-gltf/
// From https://bevy.org/examples/animation/animated-mesh/

use std::{f32::consts::*, time::Duration};

use bevy::{light::CascadeShadowConfigBuilder, prelude::*, scene::SceneInstanceReady};
use bevy_tweening::{Lens, Sequence, Tween, TweenAnim, TweeningPlugin, lens::TransformPositionLens};

use crate::GameState;


// An example asset that contains a mesh and animation.
// const GLTF_PATH: &str = "models/card_pack.gltf";
// const GLTF_PATH: &str = "models/test_cube_animation.glb";
// const GLTF_PATH: &str = "models/GenerickPack2.glb";
const GLTF_PATH: &str = "models/GenerickPack4.gltf";
// const CARD_PATH: &str = "models/card_base_model.gltf";
const _CUBE_PATH_16: &str = "models/cubeScale16.gltf";
const _CUBE_PATH_01: &str = "models/cubeScale1.gltf";
const _CUBE_PATH_08: &str = "models/cubeScale8.gltf";


const PACK_POS: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const CARP_POS: Vec3 = Vec3::new(0.0, 0.0, 0.0);


pub struct OpenCardPlugin;

impl Plugin for OpenCardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TweeningPlugin)
            .add_systems(OnEnter(GameState::OpeningPack), (
                spawn_camera,
                spawn_light,
                spawn_card_pack,
                detect_pack_animation_finished_and_spawn_cards,
                // _spawn_cube,
            ))
            // .add_systems(Update, (
            //     ddddddddddd,
            // ).run_if(in_state(GameState::OpeningPack)))
            // .add_systems(OnExit(GameState::OpeningPack), cleanup_ui);
            ;
    }
}

fn _spawn_cube(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Blockbench Scale 01
    commands.spawn((
        DespawnOnExit(GameState::OpeningPack),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(_CUBE_PATH_01),)),
        Transform::from_translation(vec3(-0.5, 0.0, 0.0)),
    ));

    // Bevy Cube
    commands.spawn((
        DespawnOnExit(GameState::OpeningPack),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(
            materials.add(StandardMaterial {
                base_color: Color::WHITE,
                // alpha_mode: AlphaMode::Mask(0.5),
                // metallic: 0.0,
                // perceptual_roughness: 1.0,
                ..default()
            })
        ),
        // RotateY,
        Transform::from_translation(vec3(0.5, 0.5, 0.0)),
        // TweenAnim::new(tween3),
    ));
}




fn spawn_camera(mut commands: Commands) {
    // Spawn camera
    commands.spawn((
        DespawnOnExit(GameState::OpeningPack),
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 40.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}

fn spawn_light(mut commands: Commands) {
    // Spawn light
    commands.spawn((
        DespawnOnExit(GameState::OpeningPack),
        DirectionalLight {
            shadows_enabled: true,
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
}

fn spawn_card_pack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {

    // Create an animation graph containing a single animation. We want the "run"
    // animation from our example asset, which has an index of two.
    let (graph, index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(GLTF_PATH)),
    );

    // Store the animation graph as an asset.
    let graph_handle = graphs.add(graph);

    // Create a component that stores a reference to our animation.
    let animation_to_play = AnimationToPlay {
        graph_handle,
        index,
    };

    // Start loading the asset as a scene and store a reference to it in a
    // SceneRoot component. This component will automatically spawn a scene
    // containing our mesh once it has loaded.
    // let mesh_scene = SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH)));

    // Spawn an entity with our components, and connect it to an observer that
    // will trigger when the scene is loaded and spawned.
    // Spawn card pack model with animation
    commands.spawn((
        DespawnOnExit(GameState::OpeningPack),
        animation_to_play,
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH),)),
        // RotateX,
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, 0.0))
            // .with_translation(vec3(0.0, -1.0, 0.0))
            .with_translation(PACK_POS)
            // .with_scale(2.0*vec3(1.0, 1.0, 1.0))
            ,
    ))
    .observe(play_animation_when_ready)
    ;

}

fn play_animation_when_ready(
    scene_ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations_to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    // The entity we spawned in `setup_mesh_and_animation` is the trigger's target.
    // Start by finding the AnimationToPlay component we added to that entity.
    if let Ok(animation_to_play) = animations_to_play.get(scene_ready.entity) {
        // The SceneRoot component will have spawned the scene as a hierarchy
        // of entities parented to our entity. Since the asset contained a skinned
        // mesh and animations, it will also have spawned an animation player
        // component. Search our entity's descendants to find the animation player.
        for child in children.iter_descendants(scene_ready.entity) {
            if let Ok(mut player) = players.get_mut(child) {
                // Tell the animation player to start the animation and keep
                // repeating it.
                //
                // If you want to try stopping and switching animations, see the
                // `animated_mesh_control.rs` example.
                // player.play(animation_to_play.index).repeat();
                player.play(animation_to_play.index);

                // Add the animation graph. This only needs to be done once to
                // connect the animation player to the mesh.
                // commands
                //     .entity(child)
                //     .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()))
                //     ;
                commands.entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()))
                    .insert(PackOpeningAnimation);
            }
        }
    }
}

#[derive(Component)]
struct PackOpeningAnimation;


fn detect_pack_animation_finished_and_spawn_cards(
    mut commands: Commands,
    // Note: We query the AnimationPlayer which is on the child entity
    players: Query<(Entity, &AnimationPlayer), With<PackOpeningAnimation>>,
    animation_data: Query<&AnimationToPlay>, 
    // Usually, AnimationToPlay is on the parent, so we might need to reference that
) {
    for (entity, player) in &players {
        // 1. Get the animation clip we are looking for
        // If AnimationToPlay is on the same entity as the player:
        if let Ok(anim_to_play) = animation_data.get(entity) {
            
            // 2. Check if the animation is finished
            // .is_finished() returns true if the animation reached the end and is not looping
            if player.animation_is_finished(anim_to_play.index) {
                info!("LotusDebug - Pack opening animation finished, spawning cards...");
                
                // Remove the marker so this doesn't trigger every frame
                commands.entity(entity).remove::<PackOpeningAnimation>();

                let card_height = 20.0;
                let card_width = 14.0;
                let card_thickness = 0.1;
    
                let start_pos = CARP_POS;
                let end_pos   = Vec3::new(0.0, 0.0, 0.5);
    
                let _tween1 = Tween::new(
                    EaseFunction::CubicOut,
                    Duration::from_secs_f32(10.0),
                    TransformPositionLens {
                        start: start_pos,
                        end: end_pos,
                    },
                );
    
                let _pull_out = Tween::new(
                    EaseFunction::QuadraticOut,
                    Duration::from_secs_f32(5.0),
                    TransformPositionLens {
                        start: Vec3::new(0.0, -1.5, -4.0),
                        end:   Vec3::new(0.0, -0.5, -4.0),
                    },
                );
                let _stretch = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs_f32(5.0),
                    TransformPositionLens {
                        start: Vec3::new(0.0, -0.5, -4.0),
                        end:   Vec3::new(0.0, 0.2, -6.0),
                    },
                );
                let _reveal = Tween::new(
                    EaseFunction::CubicOut,
                    Duration::from_secs_f32(5.0),
                    TransformPositionLens {
                        start: Vec3::new(0.0, 0.2, -6.0),
                        end:   Vec3::new(0.0, 0.0, 0.5),
                    },
                );
                let _tween2 = Sequence::new([
                    _pull_out,
                    _stretch,
                    _reveal,
                ]);
    
    
    
                let image_height = card_width - 1.;
                let image_width = card_height - 1.;
                let a = Vec3::new(0.0, 0.0, -2.5 * card_thickness);
                let b = Vec3::new(0.0, 0.0, -1.5 * card_thickness);
                let c = Vec3::new(0.0, 0.0, 0.0);
                let d = Vec3::new(0.0, 0.0, 1.5 * card_thickness);
                let e = Vec3::new(0.0, 0.0, -2.5 * card_thickness);
                for pos in [a+CARP_POS, b+CARP_POS, c+CARP_POS, d+CARP_POS, e+CARP_POS] {
                    
                    let tween3 = Tween::new(
                        EaseFunction::CubicInOut,
                        Duration::from_secs_f32(10.0),
                        BezierPositionLens {
                            p0: Vec3::new(0.0, -1.5, -4.0) * 5.0 + a,
                            p1: Vec3::new(0.0, -0.8, -4.5) * 5.0 + b,
                            p2: Vec3::new(0.0,  0.5, -6.5) * 5.0 + c,
                            p3: Vec3::new(0.0,  0.0,  0.5) * 5.0 + d,
                        },
                    );
    
                    // spawn frame of card
                    commands.spawn((
                        DespawnOnExit(GameState::OpeningPack),
                        Mesh3d(meshes.add(Cuboid::new(card_width, card_height, card_thickness))),
                        MeshMaterial3d(
                            materials.add(StandardMaterial {
                                base_color: Color::WHITE,
                                // alpha_mode: AlphaMode::Mask(0.5),
                                // metallic: 0.0,
                                // perceptual_roughness: 1.0,
                                ..default()
                            })
                        ),
                        // RotateY,
                        Transform::from_translation(pos),
                        TweenAnim::new(tween3),
                    ))
                    .with_children(|parent| {
                        
                        // spawn recto image
                        let photo_texture = asset_server.load("textures/40921678_S1J5493BMXVBDKB3RF7P22B9N0.jpeg");
                        parent.spawn((
                            DespawnOnExit(GameState::OpeningPack),
                            Mesh3d(meshes.add(Plane3d {
                                normal: Dir3::Z,
                                half_size: Vec2::new(image_height/2., image_width/2.), // 13*19
                            })),
                            MeshMaterial3d(materials.add(StandardMaterial {
                                base_color_texture: Some(photo_texture),
                                metallic: 0.0,
                                perceptual_roughness: 1.0,
                                ..default()
                            })),
                            // RotateZ,
                            Transform::from_translation(Vec3::new(0.0, 0.0, card_thickness / 2.0 + 0.001)),
                        ));
                        
                        // spawn verso image
                        let photo_texture = asset_server.load("textures/25973315_8HS551035DXVATFV2SADZRBG30.jpeg");
                        parent.spawn((
                            DespawnOnExit(GameState::OpeningPack),
                            Mesh3d(meshes.add(Plane3d {
                                normal: Dir3::Z,
                                half_size: Vec2::new(image_height/2., image_width/2.), // 13*19
                            })),
                            MeshMaterial3d(materials.add(StandardMaterial {
                                base_color_texture: Some(photo_texture),
                                metallic: 0.0,
                                perceptual_roughness: 1.0,
                                ..default()
                            })),
                            // RotateZ,
                            Transform::from_translation(Vec3::new(0.0, 0.0, -card_thickness / 2.0 - 0.001)).with_rotation(Quat::from_rotation_y(PI)), // Don't care about z-fighting
                        ));
                    });
                }
            }
        }
    }
}











// Components
// A component that stores a reference to an animation we want to play. This is
// created when we start loading the mesh (see `setup_mesh_and_animation`) and
// read when the mesh has spawned (see `play_animation_once_loaded`).
#[derive(Component)]
struct AnimationToPlay {
    graph_handle: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}

pub struct BezierPositionLens {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
}

impl Lens<Transform> for BezierPositionLens {
    fn lerp(&mut self, mut target: Mut<'_, bevy::prelude::Transform>, ratio: f32) {
        let t = ratio;
        let u = 1.0 - t;

        // cubic bezier
        let pos =
            u*u*u * self.p0 +
            3.0*u*u*t * self.p1 +
            3.0*u*t*t * self.p2 +
            t*t*t * self.p3;

        target.translation = pos;
    }
}
