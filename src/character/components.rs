use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(life: u32) -> Self {
        Self { current: life, max: life }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }

    pub fn take_damage(&mut self, amount: u32, armor: u32) {
        let actual_damage = amount.saturating_sub(armor);
        self.current = self.current.saturating_sub(actual_damage);
        actual_damage
    }

    pub fn heal(&mut self, amount: u32) {
        self.current = (self.current + amount).min(self.max);
    }
}

#[derive(Component)]
pub struct Armor(pub u32);

#[derive(Component)]
pub struct Initiative {
    pub base: u32,
    pub randomized: u32,
}

impl Initiative {
    pub fn new(base: u32) -> Self {
        Self { base, randomized: 0 }
    }
}

#[derive(Component)]
pub struct Attack(pub u32);

#[derive(Component)]
pub struct Defense(pub u32);

#[derive(Component)]
pub struct Damage(pub u32);

#[derive(Component)]
pub struct DiceRoll(pub u32);

#[derive(Component)]
pub struct SpecialAbilities {
    pub finte_level: u32,
    pub wuchtschlag_level: u32,
}

impl SpecialAbilities {
    pub fn new(finte: u32, wuchtschlag: u32) -> Self {
        Self {
            finte_level: finte,
            wuchtschlag_level: wuchtschlag,
        }
    }
}

#[derive(Component)]
pub struct CharacterType(pub String);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Combatant;