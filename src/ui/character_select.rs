use bevy::prelude::*;
use crate::game_state::GameState;
use crate::input::CharacterSelection;

#[derive(Component)]
pub struct CharSelectMarker;

#[derive(Component)]
pub struct CharSelectButton(pub usize);

#[derive(Component)]
pub struct CharSelectButtonText;

pub fn setup_character_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut char_sel: ResMut<CharacterSelection>,
) {
    let font = asset_server.load("fonts/atlantisheadbold.ttf");
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
        BackgroundColor(Color::srgb(0.10, 0.10, 0.18)),
        CharSelectMarker,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("CHARACTER"),
            TextFont {
                font: font.clone(),
                font_size: 50.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 1.0)),
        ));

        // Sword Fighter Button
        parent.spawn((
            Button,
            Node {
                margin: UiRect::all(Val::Px(8.0)),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            CharSelectButton(0),
        ))
        .with_child((
            Text::new("Schwertkrieger"),
            TextFont {
                font: font.clone(),
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            CharSelectButtonText,
        ));

        // Shield Fighter Button
        parent.spawn((
            Button,
            Node {
                margin: UiRect::all(Val::Px(8.0)),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            CharSelectButton(1),
        ))
        .with_child((
            Text::new("Schildkrieger"),
            TextFont {
                font,
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            CharSelectButtonText,
        ));
    });
}

pub fn update_character_select_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &CharSelectButton, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text, With<CharSelectButtonText>>,
    mut char_sel: ResMut<CharacterSelection>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color, btn, children) in interaction_query.iter_mut() {
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            match *interaction {
                Interaction::Hovered => {
                    // Add arrow prefix if not already there
                    if !text.0.starts_with("> ") {
                        text.0 = format!("> {}", text.0);
                    }
                    *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.25));
                }
                Interaction::None => {
                    // Remove arrow prefix
                    text.0 = text.0.trim_start_matches("> ").to_string();
                    *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
                }
                Interaction::Pressed => {
                    char_sel.current = btn.0;
                    next_state.set(GameState::Gameplay);
                }
            }
        }
    }
}