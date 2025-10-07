use bevy::prelude::*;
use bevy::ui::widget::{Button, ButtonBundle};
use crate::game_state::GameState;

#[derive(Component)]
pub struct MainMenuMarker;

#[derive(Component)]
pub struct MainMenuButton(pub usize);

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/atlantisheadbold.ttf");

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
            background_color: BackgroundColor(Color::rgb(0.12, 0.12, 0.15)),
            ..default()
        },
        MainMenuMarker,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn(TextBundle {
            text: Text::with_section(
                "DUNGEON",
                TextStyle {
                    font: font.clone(),
                    font_size: 60.0,
                    color: Color::rgb(0.0, 1.0, 0.0),
                },
                default()
            ),
            ..default()
        });

        // Start Game Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Px(45.0),
                    margin: UiRect::all(Val::Px(8.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.15, 0.15, 0.15)),
                ..default()
            },
            MainMenuButton(0),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::with_section(
                    "Neues Spiel starten",
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    default()
                ),
                ..default()
            });
        });

        // Quit Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Px(45.0),
                    margin: UiRect::all(Val::Px(8.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.15, 0.15, 0.15)),
                ..default()
            },
            MainMenuButton(1),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::with_section(
                    "Spiel beenden",
                    TextStyle {
                        font,
                        font_size: 32.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    default()
                ),
                ..default()
            });
        });
    });
}

pub fn update_main_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MainMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::rgb(0.15, 0.15, 0.15));
            }
            Interaction::Clicked => {
                match button.0 {
                    0 => next_state.set(GameState::CharacterSelection),
                    1 => std::process::exit(0),
                    _ => {}
                }
            }
        }
    }
}