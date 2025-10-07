use bevy::prelude::*;
use crate::game_state::GameState;
use crate::input::{MenuSelection};

#[derive(Component)]
pub struct MainMenuMarker;

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu_selection: ResMut<MenuSelection>,
) {
    let font = asset_server.load("fonts/atlantisheadbold.ttf");
    menu_selection.current = 0;
    menu_selection.max = 1;

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgb(0.12, 0.12, 0.15)),
        MainMenuMarker,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("DUNGEON"),
            TextFont {
                font: font.clone(),
                font_size: 60.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.0)),
        ));
        parent.spawn((
            Text::new("\n> Neues Spiel starten\n  Spiel beenden"),
            TextFont {
                font,
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            MainMenuMarker,
        ));
    });
}

pub fn update_main_menu(
    menu_selection: Res<MenuSelection>,
    mut query: Query<&mut Text, With<MainMenuMarker>>,
) {
    for mut text in query.iter_mut() {
        if text.0.contains(">") {
            text.0 = match menu_selection.current {
                0 => "> Neues Spiel starten\n  Spiel beenden".to_string(),
                1 => "  Neues Spiel starten\n> Spiel beenden".to_string(),
                _ => text.0.clone(),
            };
        }
    }
}