
use bevy::prelude::*;

use crate::GameState;
use crate::dev::components::*;
use crate::dev::observer::*;
use crate::dev::startup::*;
use crate::dev::update::*;
use crate::dev::ui::*;

pub struct DevPlaygroundPlugin;

impl Plugin for DevPlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<SwipeEvent>()
            .init_resource::<CardProgress>()
            .init_resource::<NextBatchId>()

            // Global OnAdd observers — fire when their step marker is added to
            // any entity. In practice only Pack ever receives step markers.
            // .add_observer(on_add_step_intro)
            // .add_observer(on_add_step_opening)
            // .add_observer(on_add_step_first_card)
            // .add_observer(on_add_step_card_reveal)
            // .add_observer(on_add_step_next_card)

            // Global event observers — both query Pack directly via With<Pack>
            // so no entity targeting is needed.
            // .add_observer(on_tween_batch_done)
            // .add_observer(on_player_swiped)

            // Chain ensures reset_resources runs before spawning,
            // which matters because NextBatchId must be 0 before StepIntro
            // triggers on_add_step_intro.
            // No apply_deferred needed: all spawns and the StepIntro insertion
            // share the same command flush. When OnAdd fires, Lid and Cards
            // already exist in the world.
            .add_systems(OnEnter(GameState::DevPlayground), (
                spawn_observers,
                reset_resources,
                spawn_camera,
                spawn_light,
                // Buttons ui
                spawn_buttons,
                // spawn_pack_lid,
                // spawn_cards,
                spawn_scene,
                spawn_pack_body,   // <-- spawns Pack WITH StepIntro in the bundle
            ).chain())

            // Three generic systems that know nothing about the pack sequence.
            .add_systems(Update, (
                tick_transform_tweens,
                tick_arc_tweens,   // runs after — arc wins if both exist on same entity
                dispatch_swipe,
                move_paper,
            )
            .chain()
            .run_if(in_state(GameState::DevPlayground)))
            .add_systems(OnExit(GameState::DevPlayground), despawn_observers)
            ;
    }
}


fn move_paper(
    mut query: Query<(&Name, &mut Transform)>,
) {
    for (name, mut transform) in &mut query {
        if name.as_str() == "Packs" {
            transform.rotate(Quat::from_rotation_z(0.01));
        }
    }
}


// OBSERVERS RELATED
#[derive(Component)]
struct DevObserver;
fn spawn_observers(
    mut commands: Commands,
) {
    commands.spawn((Observer::new(on_add_step_intro), DevObserver));
    commands.spawn((Observer::new(on_add_step_opening), DevObserver));
    commands.spawn((Observer::new(on_add_step_first_card), DevObserver));
    commands.spawn((Observer::new(on_add_step_card_reveal), DevObserver));
    commands.spawn((Observer::new(on_add_step_next_card), DevObserver));

    commands.spawn((Observer::new(on_tween_batch_done), DevObserver));
    commands.spawn((Observer::new(on_player_swiped), DevObserver));
}
fn despawn_observers(
    mut commands: Commands,
    q: Query<Entity, With<DevObserver>>,
) {
    for e in &q {
        commands.entity(e).despawn();
    }
}