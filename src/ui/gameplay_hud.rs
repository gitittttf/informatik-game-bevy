use bevy::prelude::*;
use crate::character::*;
use crate::combat::events::*;
use super::{StoryText, CombatLog};

#[derive(Component)]
pub struct GameplayHudMarker;

#[derive(Component)]
pub struct HpBarFill;

#[derive(Component)]
pub struct HpBarText;

#[derive(Component)]
pub struct CombatLogText;

#[derive(Component)]
pub struct StoryTextDisplay;

pub fn setup_gameplay_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/atlantisheadbold.ttf");
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        GameplayHudMarker,
    ))
    .with_children(|parent| {
        // HP Bar section
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|hp_section| {
            // HP label
            hp_section.spawn((
                Text::new("Player HP"),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            // HP bar background
            hp_section.spawn((
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(30.0),
                    margin: UiRect::top(Val::Px(5.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                BorderColor::from(Color::WHITE),
            ))
            .with_children(|bar_bg| {
                // HP bar fill
                bar_bg.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
                    HpBarFill,
                ));
            });
            // HP text (100/100)
            hp_section.spawn((
                Text::new("100 / 100"),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                HpBarText,
            ));
        });

        // Story text section
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(150.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|story_section| {
            story_section.spawn((
                Text::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.5)),
                StoryTextDisplay,
            ));
        });

        // Combat log section
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|log_section| {
            log_section.spawn((
                Text::new("--- Combat Log ---"),
                TextFont {
                    font,
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 1.0, 0.5)),
                CombatLogText,
            ));
        });
    });
}

// Update HP bar based on player health
pub fn update_hp_bar(
    player_query: Query<&Health, With<Player>>,
    mut fill_query: Query<&mut Node, With<HpBarFill>>,
    mut text_query: Query<&mut Text, With<HpBarText>>,
) {
    if let Ok(health) = player_query.single() {
        let percent = (health.current as f32 / health.max as f32) * 100.0;
        
        // Update bar width
        for mut node in fill_query.iter_mut() {
            node.width = Val::Percent(percent);
        }
        
        // Update text
        for mut text in text_query.iter_mut() {
            text.0 = format!("{} / {}", health.current, health.max);
        }
    }
}

// Update combat log with messages from events
pub fn update_combat_log(
    mut combat_log: ResMut<CombatLog>,
    mut message_events: MessageReader<CombatMessageEvent>,
    mut query: Query<&mut Text, With<CombatLogText>>,
) {
    // Add new messages
    for event in message_events.read() {
        combat_log.messages.push(event.message.clone());
        
        // Keep only last 10 messages
        if combat_log.messages.len() > 10 {
            combat_log.messages.remove(0);
        }
    }
    
    // Update display
    for mut text in query.iter_mut() {
        let log_text = format!(
            "--- Combat Log ---\n{}",
            combat_log.messages.join("\n")
        );
        text.0 = log_text;
    }
}

// Typewriter effect for story text
pub fn update_story_text_typewriter(
    time: Res<Time>,
    mut story: ResMut<StoryText>,
    mut query: Query<&mut Text, With<StoryTextDisplay>>,
) {
    if story.visible_chars < story.full_text.len() {
        story.timer.tick(time.delta());
        
        if story.timer.is_finished() {
            story.visible_chars += 1;
            story.timer.reset();
        }
        
        // Update display
        for mut text in query.iter_mut() {
            text.0 = story.full_text.chars()
                .take(story.visible_chars)
                .collect();
        }
    }
}