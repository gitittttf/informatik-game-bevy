use bevy::prelude::*;

// Event sent when combat starts
#[derive(Event)]
pub struct CombatStartEvent {
    pub enemy_count: usize,
}

// Event sent when a new round starts
#[derive(Event)]
pub struct RoundStartEvent {
    pub round_number: u32,
}

// Event sent when its the players turn
#[derive(Event)]
pub struct PlayerTurnEvent;

// Event sent when its the enemies turn
#[derive(Event)]
pub struct EnemyTurnEvent {
    pub enemy_entity: Enitty,
}

// Event for combat messages
#[derive(Event)]
pub struct CombatMessageType {
    pub message: String,
    pub message_type: CombatMessageType,
    pub delay_ms: u64,
}

// Types of combat messages (for coloring)
#[derive(Debug, Clone, Copy)]
pub enum CombatMessageType {
    RoundStart,
    PlayerAction,
    EnemyAction,
    Upgrade,
    SpecialMove,
    Damage,
    Defense,
    CombatStart,
    CombatEnd,
}

// Event when combat ends
#[derive(Event)]
pub struct CombatEndEvent {
    pub player_won: bool,
}