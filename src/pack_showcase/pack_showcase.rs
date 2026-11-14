
use bevy::prelude::*;

use crate::GameState;
use crate::pack_showcase::components::*;
use crate::pack_showcase::observer::*;
use crate::pack_showcase::startup::*;
use crate::pack_showcase::update::{tick_arc_tweens, tick_transform_tweens};
use crate::pack_showcase::ui::*;

pub struct PackShowcasePlugin;

impl Plugin for PackShowcasePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentArea>()
            .init_resource::<CardProgress>()
            .init_resource::<NextBatchId>()
            // Chain ensures reset_resources runs before spawning,
            // which matters because NextBatchId must be 0 before StepIntro
            // triggers on_add_step_intro.
            // No apply_deferred needed: all spawns and the StepIntro insertion
            // share the same command flush. When OnAdd fires, Lid and Cards
            // already exist in the world.
            .add_systems(OnEnter(GameState::PackShowcase), (
                reset_current_area,
                spawn_observers,
                reset_resources,
                spawn_camera,
                spawn_light,
                // Buttons ui
                spawn_buttons,
                spawn_scene,
                spawn_pack_body,   // <-- spawns Pack WITH StepIntro in the bundle
            ).chain())

            .add_systems(Update, (
                tick_transform_tweens,
                tick_arc_tweens,
                move_packs,
                update_status_bar,
                update_area_action_label,
            )
            .chain()
            .run_if(in_state(GameState::PackShowcase)))
            ;
    }
}


fn move_packs(
    mut query: Query<(&Name, &mut Transform)>,
) {
    for (name, mut transform) in &mut query {
        if name.as_str() == "Packs" {
            transform.rotate(Quat::from_rotation_z(0.01));
        }
    }
}


fn spawn_observers(
    mut commands: Commands,
) {
    commands.spawn((
        DespawnOnExit(GameState::PackShowcase),
        Observer::new(on_add_step_intro),
    ));
    commands.spawn((
        DespawnOnExit(GameState::PackShowcase),
        Observer::new(on_add_step_opening),
    ));
    commands.spawn((
        DespawnOnExit(GameState::PackShowcase),
        Observer::new(on_add_step_first_card),
    ));
    commands.spawn((
        DespawnOnExit(GameState::PackShowcase),
        Observer::new(on_add_step_card_reveal),
    ));
    commands.spawn((
        DespawnOnExit(GameState::PackShowcase),
        Observer::new(on_add_step_next_card),
    ));
    commands.spawn((
        DespawnOnExit(GameState::PackShowcase),
        Observer::new(on_tween_batch_done),
    ));
}

