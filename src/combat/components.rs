use bevy::prelude::*;

// Action that a character is planning to do this turn
#[derive(Component)]
pub struct QueuedAction {
    pub target: Option<Entity>, // Who to attack
    pub finte_level: u32,
    pub wuchtschlag_level: u32,
}

impl QueuedAction {
    pub fn new() -> Self {
        Self {
            taget: None,
            finte_level: 0,
            wuchtschlag_level: 0,
        }
    }
}

// Marker component - entity is currently in combat
#[derive(Component)]
pub struct InCombat;