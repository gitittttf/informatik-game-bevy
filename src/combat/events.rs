use bevy::prelude::*;

#[derive(Message)]  // Changed from Event
pub struct CombatStartEvent {
    pub enemy_count: usize,
}

#[derive(Message)]  // Changed from Event
pub struct RoundStartEvent {
    pub round_number: u32,
}

#[derive(Message)]  // Changed from Event
pub struct PlayerTurnEvent;

#[derive(Message)]  // Changed from Event
pub struct EnemyTurnEvent {
    pub enemy_entity: Entity,
}

#[derive(Message)]  // Changed from Event
pub struct CombatMessageEvent {
    pub message: String,
    pub message_type: MessageType,
    pub delay_ms: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
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

#[derive(Message)]  // Changed from Event
pub struct CombatEndEvent {
    pub player_won: bool,
}