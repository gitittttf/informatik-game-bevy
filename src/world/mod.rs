use bevy::prelude::*;
use crate::game_state::GameState;

pub mod data;
pub mod resources;
pub mod gameplay_systems;

pub use data::*;
pub use resources::*;
pub use gameplay_systems::*;

pub fn plugin(app: &mut App) {
    app
        .init_resource::<WorldState>()
        
        // Gameplay systems
        .add_systems(OnEnter(GameState::Gameplay), (
            spawn_player_system,
            load_room_system,
        ))
        .add_systems(Update, (
            start_combat_when_ready_system,
        ).run_if(in_state(GameState::Gameplay)))
        
        // Combat aftermath systems
        .add_systems(Update, (
            handle_combat_end_system,
            apply_upgrades_system,
        ).run_if(in_state(GameState::Combat)))
        
        // Cleanup when leaving gameplay
        .add_systems(OnExit(GameState::Gameplay), cleanup_player_system);
}

// Cleanup player when returning to menu
fn cleanup_player_system(
    mut commands: Commands,
    player_query: Query<Entity, With<crate::character::Player>>,
    enemy_query: Query<Entity, With<crate::character::Enemy>>,
) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
}