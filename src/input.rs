use bevy::prelude::*;
use crate::game_state::GameState;
use crate::combat::events::PlayerTurnEvent;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_menu_input.run_if(in_state(GameState::MainMenu)))
            .add_systems(Update, handle_character_select_input.run_if(in_state(GameState::CharacterSelection)))
            .add_systems(Update, handle_gameplay_input.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, handle_combat_input.run_if(in_state(GameState::Combat)))
            .add_systems(Update, handle_settings_input.run_if(in_state(GameState::Settings)));
    }
}

// Menu input handling
fn handle_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut menu_selection: ResMut<MenuSelection>,
) {
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        menu_selection.previous();
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        menu_selection.next();
    }
    if keyboard.just_pressed(KeyCode::Enter) {
        match menu_selection.current() {
            0 => next_state.set(GameState::CharacterSelection),
            1 => next_state.set(GameState::Settings),
            2 => std::process::exit(0), // Exit game
            _ => {}
        }
    }
}

// Character selection input
fn handle_character_select_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut char_selection: ResMut<CharacterSelection>,
) {
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        char_selection.previous();
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        char_selection.next();
    }
    if keyboard.just_pressed(KeyCode::Enter) {
        // Character selected, start game
        next_state.set(GameState::Gameplay);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

// Gameplay/exploration input
fn handle_gameplay_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut show_map: ResMut<ShowMap>,
) {
    // M key toggles map
    if keyboard.just_pressed(KeyCode::KeyM) {
        show_map.0 = !show_map.0;
    }
    
    // Enter to start combat or advance to next room
    if keyboard.just_pressed(KeyCode::Enter) {
        // This will be handled by game logic systems
    }
    
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

// Combat input handling
fn handle_combat_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut combat_selection: ResMut<CombatSelection>,
    mut player_turn_events: MessageWriter<PlayerTurnEvent>,
) {
    // Arrow keys to navigate actions/enemies
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        combat_selection.previous_action();
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        combat_selection.next_action();
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        combat_selection.previous_enemy();
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        combat_selection.next_enemy();
    }
    
    // Number keys for Finte/Wuchtschlag levels (0-3)
    if keyboard.just_pressed(KeyCode::Digit0) {
        combat_selection.set_ability_level(0);
    }
    if keyboard.just_pressed(KeyCode::Digit1) {
        combat_selection.set_ability_level(1);
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        combat_selection.set_ability_level(2);
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        combat_selection.set_ability_level(3);
    }
    
    // Enter to confirm action
    if keyboard.just_pressed(KeyCode::Enter) {
        player_turn_events.write(PlayerTurnEvent);
    }
}

// Settings input
fn handle_settings_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

// Resources for UI state
#[derive(Resource)]
pub struct MenuSelection {
    pub current: usize,
    pub max: usize,
}

impl Default for MenuSelection {
    fn default() -> Self {
        Self { current: 0, max: 2 }
    }
}

impl MenuSelection {
    pub fn current(&self) -> usize {
        self.current
    }
    
    pub fn next(&mut self) {
        self.current = (self.current + 1) % (self.max + 1);
    }
    
    pub fn previous(&mut self) {
        self.current = if self.current == 0 { self.max } else { self.current - 1 };
    }
}

#[derive(Resource)]
pub struct CharacterSelection {
    pub current: usize,
    pub max: usize,
}

impl Default for CharacterSelection {
    fn default() -> Self {
        Self { current: 0, max: 1 } // Sword Fighter, Shield Fighter
    }
}

impl CharacterSelection {
    pub fn current(&self) -> usize {
        self.current
    }
    
    pub fn next(&mut self) {
        self.current = (self.current + 1) % (self.max + 1);
    }
    
    pub fn previous(&mut self) {
        self.current = if self.current == 0 { self.max } else { self.current - 1 };
    }
    
    pub fn get_player_type(&self) -> PlayerType {
        match self.current {
            0 => PlayerType::SwordFighter,
            1 => PlayerType::ShieldFighter,
            _ => PlayerType::SwordFighter,
        }
    }
}

#[derive(Resource, Default)]
pub struct ShowMap(pub bool);

#[derive(Resource)]
pub struct CombatSelection {
    pub selected_action: usize,
    pub selected_enemy: usize,
    pub enemy_count: usize,
    pub finte_level: u32,
    pub wuchtschlag_level: u32,
}

impl Default for CombatSelection {
    fn default() -> Self {
        Self {
            selected_action: 0,
            selected_enemy: 0,
            enemy_count: 0,
            finte_level: 0,
            wuchtschlag_level: 0,
        }
    }
}

impl CombatSelection {
    pub fn next_action(&mut self) {
        self.selected_action = (self.selected_action + 1) % 2; // Attack or abilities
    }
    
    pub fn previous_action(&mut self) {
        self.selected_action = if self.selected_action == 0 { 1 } else { 0 };
    }
    
    pub fn next_enemy(&mut self) {
        if self.enemy_count > 0 {
            self.selected_enemy = (self.selected_enemy + 1) % self.enemy_count;
        }
    }
    
    pub fn previous_enemy(&mut self) {
        if self.enemy_count > 0 {
            self.selected_enemy = if self.selected_enemy == 0 { 
                self.enemy_count - 1 
            } else { 
                self.selected_enemy - 1 
            };
        }
    }
    
    pub fn set_ability_level(&mut self, level: u32) {
        // This would be context-dependent (Finte or Wuchtschlag)
        // For now, just store it
        if self.selected_action == 1 {
            self.finte_level = level;
        } else if self.selected_action == 2 {
            self.wuchtschlag_level = level;
        }
    }
}

// Placeholder for PlayerType enum (should match your world/data.rs)
#[derive(Debug, Clone, Copy)]
pub enum PlayerType {
    SwordFighter,
    ShieldFighter,
}