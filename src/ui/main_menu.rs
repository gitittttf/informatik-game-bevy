use bevy::prelude::*;
use crate::game_state::GameState;

#[derive(Component)]
pub struct MainMenuMarker;

#[derive(Component)]
pub struct MainMenuButton(pub usize);

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/atlantisheadbold.ttf");

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
        // Title
        parent.spawn((
            Text::new("DUNGEON"),
            TextFont {
                font: font.clone(),
                font_size: 60.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.0)),
        ));

        // Start Game Button
        parent.spawn((
            Button,
            Node {
                width: Val::Auto,
                height: Val::Px(45.0),
                margin: UiRect::all(Val::Px(8.0)),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            MainMenuButton(0),
        ))
        .with_child((
            Text::new("Neues Spiel starten"),
            TextFont {
                font: font.clone(),
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));

        // Quit Button
        parent.spawn((
            Button,
            Node {
                width: Val::Auto,
                height: Val::Px(45.0),
                margin: UiRect::all(Val::Px(8.0)),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            MainMenuButton(1),
        ))
        .with_child((
            Text::new("Spiel beenden"),
            TextFont {
                font,
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
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
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
            Interaction::Pressed => {
                match button.0 {
                    0 => next_state.set(GameState::CharacterSelection),
                    1 => std::process::exit(0),
                    _ => {}
                }
            }
        }
    }
}