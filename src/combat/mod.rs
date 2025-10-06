use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod events;
pub mod systems;

// re-export so other modules can use them easily
pub use components::*;
pub use resources::*;
pub use events::*;
pub use systems::*;

use crate::game_state::GameState;

// combat plugin, registers everything
pub fn plugin(app: &mut App) {
    app
        // add events
        .add_event::<CombatStartEvent>()
        .add_event::<RoundStartEvent>()
        .add_event::<PlayerTurnEvent>()
        .add_event::<EnemyTurnEvent>()
        .add_event::<CombatMessageEvent>()
        .add_event::<CombatEndEvent>()

        // add resources
        .init_resource::<BattleState>()

        // Systems that run when in combat state
        .add_systems(Update, (
            process_turn_system,
            enemy_ai_system,
            execute_attack_system,
            check_victory_system,
        ).run_if(in_state(GameState::Combat)))

        // system that runs when entering combat state
        .add_systems(OnEnter(GameState::Combat), start_combat_system);
}