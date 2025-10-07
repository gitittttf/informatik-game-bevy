use bevy::{input_focus::InputFocus, prelude::*, window::PresentMode};
use informatik_game_bevy::{GamePlugin, InputPlugin, GameState}; 

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                title: "Dungeon Game".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<InputFocus>()
        .init_state::<GameState>()
        .add_plugins(GamePlugin)
        .add_plugins(InputPlugin)
        .run();
}