use bevy::prelude::*;
use crate::input::CharacterSelection;

#[derive(Component)]
pub struct CharSelectMarker;

pub fn setup_character_select(mut commands: Commands, mut char_sel: ResMut<CharacterSelection>) {
    char_sel.current = 0;
    char_sel.max = 1;
    
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
        CharSelectMarker,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("CHARACTER"),
            TextFont {
                font_size: 50.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 1.0)),
        ));
        
        parent.spawn((
            Text::new("\n► Schwertkrieger\n  Schildkrieger"),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::top(Val::Px(30.0)),
                ..default()
            },
        ));
    });
}

pub fn update_character_select(
    char_sel: Res<CharacterSelection>,
    mut query: Query<&mut Text, With<CharSelectMarker>>,
) {
    for mut text in query.iter_mut() {
        if text.0.contains("►") {
            text.0 = match char_sel.current {
                0 => "► Schwertkrieger\n  Schildkrieger".to_string(),
                1 => "  Schwertkrieger\n► Schildkrieger".to_string(),
                _ => text.0.clone(),
            };
        }
    }
}