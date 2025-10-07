use bevy::prelude::*;
use crate::game_state::GameState;
use crate::input::{MenuSelection};

#[derive(Component)]
pub struct MainMenuMarker;

pub fn setup_main_menu(mut commands: Commands, mut menu_selection: ResMut<MenuSelection>) {
    menu_selection.current = 0;
    menu_selection.max = 2;
    
    // Root UI node
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        MainMenuMarker,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("DUNGEON"),
            TextFont {
                font_size: 60.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.0)),
        ));
        
        // Menu options
        parent.spawn((
            Text::new("\n► Neues Spiel starten\n  Spiel beenden"),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::top(Val::Px(50.0)),
                ..default()
            },
        ));
    });
}

pub fn update_main_menu(
    menu_selection: Res<MenuSelection>,
    mut query: Query<&mut Text, With<MainMenuMarker>>,
) {
    for mut text in query.iter_mut() {
        if text.0.contains("►") {
            text.0 = match menu_selection.current {
                0 => "> Neues Spiel starten\n  Spiel beenden".to_string(),
                1 => "  Neues Spiel starten\n> Spiel beenden".to_string(),
                _ => text.0.clone(),
            };
        }
    }
}