use bevy::prelude::*;
use rand::Rnd;

use crate::character::*;
use super::components::*;
use super::resources::*;
use super::events::*;

// System that starts a combat encounter
pub fn start_combat_system(
    mut commands: Commands,
    mut battle_state: ResMut<BattleState>,
    mut combat_start_events: EventWriter<CombatStartEvent>,
    // Query gets all entities with Player component + their other components
    mut player_query: Query<(Enitity, &mut Initiative), With<Player>>,
    mut enemy_query: Query<(Entity, &mut Initiative), With<Enemy>>,
) {
    // only run if combat is starting
    if !battle_state.combat_active {
        return;
    }

    // Randomize initiative (w6 + base initiative)
    let mut rng = rand::thread_rng();

    for (entity, mut initiative) in player_query.iter_mut() {
        let roll = rng.gen_range(1..=6);
        initiative.randomized = initiative.base + roll;
    }

    for (entity, mut initiative) in enemy_query.iter_mut() {
        let roll = rng.gen_range(1..=6);
        initiative.randomized = initiative.base + roll;
    }

    // Build turn order based on initiative
    let mut all_combatants: Vec<(Entity, u32)> = Vec::new();

    for (entity, initiative) in player_query.iter() {
        all_combatants.push((entity, initiative.randomized));
    }

    for (entity, initiative) in enemy_query.iter() {
        all_combatants.push((entity, initiative.randomized));
    }

    // Sort by initative (highest first)
    all_combatants.sort_by(|a, b| b.1.cmp(&a.1));

    // Update battle state with turn order
    battle_state.turn_queue = all_combatants.iter().map(|(e, _)| *e).collect();
    battle_state.current_turn_index = 0;
    battle_state.current_round = 1;

    // Send event that combat started
    combat_start_events.send(CombatStartEvent {
        enemy_count: enemy_query.iter().count(),
    });
}

pub fn process_turn_system(
    battle_state: Res<BattleState>,
    mut round_events: EventWriter<RoundStartEvent>,
    mut player_turn_events: EventWriter<PlayerTurnEvent>,
    mut enemy_turn_events: EventWriter<EnemyTurnEvent>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if !battle_state.combat_active {
        return;
    }

    // Check if round is over
    if battle_state.is_round_over() {
        // WIll be handled by start_new_round_system
        return;
    }

    // Get current turn entity
    if let Some(current_entity) = battle_state.current_turn() {
        // Check if its a player
        if player_query.get(current_entity).is_ok() {
            if !battle_state.waiting_for_player_input {
                player_turn_events.send(PlayerTurnEvent);
            }
        }
        // Ckeck if its an enemy
        else if enemy_query.get(current_entity).is_ok() {
            enemy_turn_events.send(EnemyTurnEvent {
                enemy_entity: current_entity,
            });
        }
    }
}

// system that executes an attack
pub fn execute_attack_system(
    mut commmands: Commands,
    mut message_events: EventWriter<CombatMessageEvent>,
    attacker_query: Query<(
        &CharacterType,
        &Attack,
        &Damage,
        &DiceRoll,
        &SpecialAbilities,
    )>,
    mut defender_query: Query<(
        &CharacterType,
        &mut Health,
        &Armor,
        &Defense,
    )>,
    action_query: Query<(Entity, &QueuedAction)>,
) {
    // Find entities with queued actions
    for (attacker_entity, action) in action_query.iter() {
        // Make sure they have a target
        let Some(target_entity) = action.target else {
            continue;
        };

        // Get attacker stats
        let Ok((
            attacker_type,
            attack_stat,
            damage_stat,
            dice_roll,
            abilities,
        )) = attacker_query.get(attacker_entity) else {
            continue;
        };

        // Get defender stats
        let Ok((
            defender_type,
            mut defender_health,
            defender_armor,
            defender_defense,
        )) = defender_query.get_mut(target_entity) else {
            continue;
        };

        // Roll w20 for attack
        let mut rng = rand::thread_rng();
        let attack_roll = rng.gen_range(1..=20);

        // Calculate attack threshold (harder with special moves)
        let finte = action.finte_level.min(abilities.finte_level);
        let wuchtschlag = action.wuchtschlag_level.min(abilities.wuchtschlag_level);
        let attack_threshold = attack_stat.0
            .saturating_sub(finte)
            .saturating_sub(wuchtschlag * 2);

        // Send attack start message
        message_events.send(CombatMessageEvent {
            message: format!("{} startet den Angriff!", attacker_type.0),
            message_type: CombatMessageType::PlayerAction,
            delay_ms: 0,
        });

        // Check if attack hits
        if attack_roll <= attack_threshold {
            // attack hits
            let mut total_damage = damage.stat.0;

            // roll damage dice
            for _ in 0..dice_roll.0 {
                total_damage += rng.gen_range(1..=20);
            }

            // bonus damage from wuchtschlag
            total_damage += wuchtschlag * 2;

            // defender tries defending
            let defense_roll = rng.gen_range(1..=20);
            let defense_debuff = finte * 2;
            let defense_threshold = defender_defense.0.saturating_sub(defense_debuff);

            if defense_roll > defense_threshold {
                // defense failed, take damage
                let actual_damage = total_damage.saturating_sub(defender_armor.0);
                defender_health.take_damage(actual_damage, 0);

                message_events.send(CombatMessageEvent {
                    message: format!("{} nimmt {} Schaden!", defender_type.0, actual_damage),
                    message_type: CombatMessageType::Damage,
                    delay_ms: 300,
                });
            } else {
                // defense success
                message_events.send(CombatMessageEvent {
                    message: format!("{} parriert erfolgreich!", defender_type.0),
                    message_type: CombatMessageType::Defense,
                    delay_ms: 200,
                });
            }
        } else {
            // attack missed
            message_events.send(CombatMessageEvent {
                    message: format!("{} scheiterte anzugreifen...", attacker_type.0),
                    message_type: CombatMessageType::PlayerAction,
                    delay_ms: 0,
            });
        }

        // remove queued action (already executed)
        commands.entity(attacker_entity).remove::<QueuedAction>();

    }

}

pub fn enemy_ai_system(
    mut commands: Commands,
    battle_state: Res<BattleState>,
    mut enemy_turn_events: EventReader<EnemyTurnEvent>,
    enemy_query: Query<&SpecialAbilities, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
) {
    for event in enemy_turn_events.read() {
        let enemy_entity = event.enemy_entity;

        // Get enemys abilities
        let Ok(abilities) = enemy_query.get(enemy_entity) else {
            continue;
        };

        // Get player entity (target)
        let Ok(player_entity) = player_query.get_single() else {
            continue;
        };

        // Randomly choose finte and wuchtschlag levels
        let mut rng = rang::thread_rng();
        let finte = rng.gen_range(0..=abilities.finte_level);
        let wuchtschlag = rng.gen_range(0..=abilities.wuchtschlag_level);
        
        // Add action to enemy
        commands.entity(enemy_entity).insert(QueuedAction {
            target: Some(player_entity),
            finte_level: finte,
            wuchtschlag_level: wuchtschlag,
        });
    }

}

// system that checks if combat should end
pub fn check_victory_system(
    mut battle_state: ResMut<BattleState>,
    mut combat_end_events: EventWriter<CombatEndEvent>,
    player_query: Query<&Health, With <Player>>,
    enemy_query: Query<&Health, With<Enemy>>,
) {
    if !battle_state.combat_active {
        return;
    }

    // check if player dead
    if let Ok(player_health) = player_query.get_single() {
        if !player_health.is_alive() {
            battle_state.combat_active = false;
            combat_end_events.send(CombatEndEvent { player_won: false });
        }
    };

    // check if all enemies are dead
    let enemies_alive = enemy_query.iter().any(|health| health.is_alive());
    if !enemies_alive {
        battle_state.combat_active = false;
        combat_end_events.send(CombatEndEvent { player_won: true });
    }
}