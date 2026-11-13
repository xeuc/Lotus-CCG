use bevy::prelude::*;

use crate::other_sppd::components::*;


pub fn walking_state_system(
    mut states: crate::other_sppd::components::States,
    mut query: Query<(
        Entity,
        &mut Transform,
        &AttackRange,
        // cannot use this as my all shape querry will not contain all entities
        // and I don't want the current entities to walk to only entities that are in "walking" state!
        // &Walking,
        Option<&Walking>,
    ), With<Shape>>,
    time: Res<Time>,
) {
    let all_shapes: Vec<(Entity, Transform)> = query.iter().map(|(e, t, _, _)| (e, *t)).collect();

    for (entity, mut transform, atk_range, walking) in query.iter_mut() {

        // i need walking step here
        let Some(walking) = walking else {
            continue;
        };

        // target is..
        let target = walking.0;

        let Some((_, other_transform)) = all_shapes.iter().find(|(e, _)| *e == target) else {
            continue;
        };

        let dir = other_transform.translation - transform.translation;
        let distance_squared = dir.length_squared();
        let other_rayon = other_transform.scale.x / 2.0;

        match distance_squared {
            d if d > (atk_range.0 + other_rayon).powi(2) => { transform.translation += dir.normalize() * 1.0 * time.delta_secs(); }
            d if d > (transform.scale.x / 2.0 + other_rayon).powi(2) => { states.entity(entity).transition(walking, Attacking(target)); }
            _ => { states.entity(entity).transition(walking, Colliding {}); }
        }
    }
}
