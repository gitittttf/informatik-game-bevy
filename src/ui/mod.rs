use bevy::prelude::*;
use crate::game_state::GameState;

pub mod main_menu;
pub mod character_select;
pub mod gameplay_hud;

pub use main_menu::*;
pub use character_select::*;
pub use gameplay_hud::*;

pub fn plugin(app: &mut App) {
    app
        // Main menu systems
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(Update, update_main_menu_buttons.run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), cleanup_menu::<MainMenuMarker>)
        
        // Character selection systems
        .add_systems(OnEnter(GameState::CharacterSelection), setup_character_select)
        .add_systems(Update, update_character_select_buttons.run_if(in_state(GameState::CharacterSelection)))
        .add_systems(OnExit(GameState::CharacterSelection), cleanup_menu::<CharSelectMarker>)
        
        // Gameplay HUD systems
        .add_systems(OnEnter(GameState::Gameplay), setup_gameplay_hud)
        .add_systems(Update, (
            update_hp_bar,
            update_combat_log,
            update_story_text_typewriter,
        ).run_if(in_state(GameState::Gameplay)))
        .add_systems(OnExit(GameState::Gameplay), cleanup_menu::<GameplayHudMarker>);
}

// Generic cleanup system
fn cleanup_menu<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Resources
#[derive(Resource)]
pub struct StoryText {
    pub full_text: String,
    pub visible_chars: usize,
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct CombatLog {
    pub messages: Vec<String>,
}