use bevy::prelude::*;

use crate::other_sppd::components::*;

pub fn idle_state_system(
    mut states: crate::other_sppd::components::States, 
    mut shapes: Query<(
        Entity,
        &mut Transform,
        &Team,
        &AttackRange,
        &DetectionRange,
        Option<&Idle>,
    ), With<Shape>>,
    time: Res<Time>,
) {
    let all_shapes: Vec<(Entity, Transform, Team)> = shapes.iter().map(|(e, tr, te, _, _, _)| (e, *tr, *te)).collect();

    for (entity, mut transform, team, atk_range, dtk_range, idle_0) in shapes.iter_mut() {

        let Some(idle) = idle_0 else {
            continue;
        };

        let mut closest_enemy_and_distance_squared: Option<(Entity, f32, f32)> = None; // (entity, distance_squared, rayon_other)

        for (other_entity, other_transform, other_team) in all_shapes.iter() {
            // not itself
            if entity == *other_entity {
                continue;
            }

            let distance_squared = transform.translation.distance_squared(other_transform.translation);

            // Filter same team => ranges have a sens only about others team shapes...........
            if team == other_team {
                continue;
            }

            if closest_enemy_and_distance_squared.is_none() || distance_squared < closest_enemy_and_distance_squared.unwrap().1 {
                closest_enemy_and_distance_squared = Some((*other_entity, distance_squared, other_transform.scale.x/2.0));
            }
        }

        let dir = Vec3::new(idle.0, 0.0, 0.0);

        // Do the math on the closest entity only
        if let Some((closest_target, distance_squared, other_rayon)) = closest_enemy_and_distance_squared {

            match distance_squared {
                // ranges take in account self size, but not the other's shape size oc
                d if d > (dtk_range.0 + other_rayon).powi(2) => { /* still idle, retry in 2 sec */ transform.translation += dir * 1.0 * time.delta_secs(); },
                d if d > (atk_range.0 + other_rayon).powi(2) => { /* detecting */ states.entity(entity).transition(idle, Walking(closest_target)); },
                d if d > (transform.scale.x/2.0 + other_rayon).powi(2) => { /* attacking */ states.entity(entity).transition(idle, Attacking(closest_target)); },
                _ => { /* colliding, should not reach that */ states.entity(entity).transition(idle, Colliding {}); },
            }
        } else {

            transform.translation += dir * 1.0 * time.delta_secs();
        }
        

    }

}
