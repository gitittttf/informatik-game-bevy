use bevy::prelude::*;
use bevy::ui::*;
use crate::game_state::GameState;
use crate::input::CharacterSelection;

#[derive(Component)]
pub struct CharSelectMarker;

#[derive(Component)]
pub struct CharSelectButton(pub usize);

pub fn setup_character_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut char_sel: ResMut<CharacterSelection>,
) {
    let font = asset_server.load("fonts/atlantisheadbold.ttf");
    char_sel.current = 0;
    char_sel.max = 1;

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.10, 0.10, 0.18)),
            ..default()
        },
        CharSelectMarker,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "CHARACTER",
            TextStyle {
                font: font.clone(),
                font_size: 50.0,
                color: Color::srgb(0.0, 1.0, 1.0),
            },
        ));

        // Sword Fighter Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(8.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ..default()
            },
            CharSelectButton(0),
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                "Schwertkrieger",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        });

        // Shield Fighter Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(8.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ..default()
            },
            CharSelectButton(1),
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                "Schildkrieger",
                TextStyle {
                    font,
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        });
    });
}

// System für Hover und Klick
pub fn update_character_select_buttons(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &CharSelectButton, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut char_sel: ResMut<CharacterSelection>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color, btn, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Hovered => {
                text.0 = format!("> {}", text.0.trim_start_matches("> "));
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.25));
            }
            Interaction::None => {
                text.0 = text.0.trim_start_matches("> ").to_string();
                *bg_color = BackgroundColor(Color::srgb(0.10, 0.10, 0.18));
            }
            Interaction::Pressed => {
                char_sel.current = btn.0;
                next_state.set(GameState::Gameplay);
            }
        }
    }
}