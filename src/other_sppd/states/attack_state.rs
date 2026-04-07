
use bevy::prelude::*;

use crate::other_sppd::components::*;


// note: like walk state, this state is team blinded
// actually no i need it to know which direction i need to go after defeating an enemy
pub fn attack_state_system(
    mut states: crate::other_sppd::components::States, 
    time: Res<Time>,
    mut attackers: Query<(
        Entity,
        &mut AttackCooldown,
        &Attack,
        &Attacking,
        &Team,
    )>,
    mut lives: Query<&mut Life>,
) {
    for (_ent, mut cooldown, atk, state, team) in attackers.iter_mut() {
        
        // no target mean enemy defeated! 
        if lives.get_mut(state.0).is_err() {
            match team {
                Team::Blue => states.entity(_ent).transition(state, Idle(-1.0)),
                Team::Red => states.entity(_ent).transition(state, Idle(1.0)),
            }
            
            cooldown.timer.reset();
            continue;
        }

        cooldown.timer.tick(time.delta());
        
        if cooldown.timer.is_finished() {
            // the target entity is located in the attack state:
            let target = state.0;
            if let Ok(mut life) = lives.get_mut(target) {
                life.hp -= atk.degat;
            }
        }
    }
}





pub fn kill_system(
    mut commands: Commands,
    // mut events: MessageWriter<DeathEvent>,
    query: Query<(Entity, &Life)>,
) {
    for (e, life) in &query {
        if life.hp <= 0.0 {
            // events.write(DeathEvent(e));
            commands.entity(e).despawn();
        }
    }
}

// pub fn on_death_system(
//     mut events: MessageReader<DeathEvent>,
//     mut attackers: Query<(&AttackTarget, &mut MotionState)>,
// ) {
//     for DeathEvent(dead_entity) in events.read() {
//         for (target, mut state) in attackers.iter_mut() {
//             if target.0 == *dead_entity {
//                 // target dead
//                 *state = MotionState::Idle;
//             }
//         }
//     }
// }



// pub fn set_attacking(mut query: Query<&mut MotionState>) {
//     for mut state in query.iter_mut() {
//         if some_condition {
//             *state = MotionState::Attacking;
//         }
//     }
// }

// pub fn on_enter_attack(
//     query: Query<
//         (Entity, &MotionState),
//         Changed<MotionState>
//     >,
// ) {
//     for (entity, state) in query.iter() {
//         if *state == MotionState::Attacking {
//             println!(">>> Entity {entity:?} débute une attaque !");
//         }
//     }
// }

