use bevy::prelude::*;
use super::components::*;


// Bundle to spawn a player with all stats
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub combatant: Combatant,
    pub character_type: CharacterType,
    pub health: Health,
    pub armor: Armor,
    pub initiative: Initiative,
    pub attack: Attack,
    pub defense: Defense,
    pub damage: Damage,
    pub dice: DiceRoll,
    pub abilities: SpecialAbilities,
}

impl PlayerBundle {
    pub fn new(
        life_total: u32,
        armor_value: u32,
        initiative: u32,
        attack: u32,
        defense: u32,
        damage: u32,
        num_w6: u32,
        finte_level: u32,
        wuchtschlag_level: u32,
    ) -> Self {
        Self {
            player: Player,
            combatant: Combatant,
            character_type: CharacterType("Spieler".to_string()),
            health: Health::new(life_total),
            armor: Armor(armor_value),
            initiative: Initiative::new(initiative),
            attack: Attack(attack),
            defense: Defense(defense),
            damage: Damage(damage),
            dice: DiceRoll(num_w6),
            abilities: SpecialAbilities::new(finte_level, wuchtschlag_level),
        }
    }
}

// Bundle to spawn an enemy with all stats
#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub combatant: Combatant,
    pub character_type: CharacterType,
    pub health: Health,
    pub armor: Armor,
    pub initiative: Initiative,
    pub attack: Attack,
    pub defense: Defense,
    pub damage: Damage,
    pub dice: DiceRoll,
    pub abilities: SpecialAbilities,
}

impl EnemyBundle {
    pub fn new(
        enemy_type: String,
        life_total: u32,
        armor_value: u32,
        initiative: u32,
        attack: u32,
        defense: u32,
        damage: u32,
        num_w6: u32,
        finte_level: u32,
        wuchtschlag_level: u32,
    ) -> Self {
        Self {
            enemy: Enemy,
            combatant: Combatant,
            character_type: CharacterType(enemy_type),
            health: Health::new(life_total),
            armor: Armor(armor_value),
            initiative: Initiative::new(initiative),
            attack: Attack(attack),
            defense: Defense(defense),
            damage: Damage(damage),
            dice: DiceRoll(num_w6),
            abilities: SpecialAbilities::new(finte_level, wuchtschlag_level),
        }
    }
}