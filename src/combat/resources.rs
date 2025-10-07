use bevy::prelude::*;

// Current state of the battle
#[derive(Resource, Default)]
pub struct BattleState {
    pub current_round: u32,
    pub turn_queue: Vec<Entity>, // List of who goes in what order
    pub current_turn_index: usize, // Whose turn it is right now
    pub waiting_for_player_input: bool, // Waiting for player to choose?
    pub combat_active: bool, // Is combat happening?
}

impl BattleState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current_turn(&self) -> Option<Entity> {
        self.turn_queue.get(self.current_turn_index).copied()
    }

    pub fn advance_turn(&mut self) {
        self.current_turn_index += 1;
    }

    pub fn is_round_over(&self) -> bool {
        self.current_turn_index >= self.turn_queue.len()
    }
}