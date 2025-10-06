use bevy::prelude::*;
use informatik_game_bevy::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Window, rendering und input
        .add_plugins(GamePlugin) // Game logik
        .run(); // Bevy's game loop
}