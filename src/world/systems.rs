use bevy::prelude::*;
use crate::character::*;
use super::data::*;
use super::resources::*;

// system for spawning enemies for current room
pub fn spawn_room_enemies_system(
    mut commands: Commands,
    world: Res<WorldState>,
) {
    let room = world.current_room();
    
    for enemy_type in room.enemies() {
        let stats = enemy_type.stats();
        
        commands.spawn(EnemyBundle::new(
            stats.9.to_string(),  // name
            stats.0,  // life
            stats.1,  // armor
            stats.2,  // initiative
            stats.3,  // attack
            stats.4,  // defense
            stats.5,  // damage
            stats.6,  // numW6
            stats.7,  // finte
            stats.8,  // wuchtschlag
        ));
    }
}

// System to apply room upgrade
pub fn apply_room_upgrade_system(
    world: Res<WorldState>,
    mut player_query: Query<(
        &mut Health,
        &mut Armor,
        &mut Attack,
        &mut Defense,
        &mut Damage,
        &mut SpecialAbilities,
    ), With<Player>>,
) {
    let room = world.current_room();
    
    if let Some(upgrade) = UpgradeTypeData::for_room(room) {
        let stats = upgrade.stats();
        
        for (mut health, mut armor, mut attack, mut defense, mut damage, mut abilities) in player_query.iter_mut() {
            health.current += stats.0;
            health.max += stats.1;
            armor.0 += stats.2;
            attack.0 += stats.4;
            defense.0 += stats.5;
            damage.0 += stats.6;
            abilities.finte_level += stats.7;
            abilities.wuchtschlag_level += stats.8;
        }
    }
}