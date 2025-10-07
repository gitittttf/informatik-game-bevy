use bevy::prelude::*;
use informatik_game_bevy::{GamePlugin, InputPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Window, rendering und input
        .add_plugins(GamePlugin) // Game logik
        .add_plugins(InputPlugin) // crate-local input handling
        .run(); // Bevy's game loop
}