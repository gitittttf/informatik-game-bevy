use bevy::prelude::*;
use crate::character::*;
use crate::combat::events::*;
use crate::world::WorldState;
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

#[derive(Component)]
pub struct RoomInfoText;

#[derive(Component)]
pub struct PlayerStatsText;

pub fn setup_gameplay_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/atlantisheadbold.ttf");
    
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
        GameplayHudMarker,
    ))
    .with_children(|parent| {
        // Top bar: Room info and player stats
        parent.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Start,
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        })
        .with_children(|top_bar| {
            // Left: Room info
            top_bar.spawn(Node {
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|left| {
                left.spawn((
                    Text::new("Raum 1 von 8"),
                    TextFont {
                        font: font.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 1.0)),
                    RoomInfoText,
                ));
            });
            
            // Right: Player stats
            top_bar.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::End,
                ..default()
            })
            .with_children(|right| {
                right.spawn((
                    Text::new("Stats werden geladen..."),
                    TextFont {
                        font: font.clone(),
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.9, 0.7)),
                    PlayerStatsText,
                ));
            });
        });
        
        // HP Bar section (visually improved)
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(15.0)),
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            BorderColor::all(Color::srgb(0.3, 0.3, 0.4)),
        ))
        .with_children(|hp_container| {
            // HP label
            hp_container.spawn((
                Text::new("♥ GESUNDHEIT"),
                TextFont {
                    font: font.clone(),
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.3, 0.3)),
                Node {
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
            ));
            
            // HP bar background
            hp_container.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(35.0),
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.05, 0.05)),
                BorderColor::all(Color::srgb(0.5, 0.2, 0.2)),
            ))
            .with_children(|bar_bg| {
                // HP bar fill
                bar_bg.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                    HpBarFill,
                ));
            });
            
            // HP text
            hp_container.spawn((
                Text::new("100 / 100"),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::top(Val::Px(8.0)),
                    ..default()
                },
                HpBarText,
            ));
        });

        // Story text section (main focus)
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(200.0),
                padding: UiRect::all(Val::Px(20.0)),
                margin: UiRect::bottom(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
            BorderColor::all(Color::srgb(0.5, 0.5, 0.6)),
        ))
        .with_children(|story_section| {
            story_section.spawn((
                Text::new("Lade Geschichte..."),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.95, 0.7)),
                StoryTextDisplay,
            ));
        });

        // Combat log section
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(15.0)),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.08, 0.05)),
            BorderColor::all(Color::srgb(0.2, 0.4, 0.2)),
        ))
        .with_children(|log_section| {
            log_section.spawn((
                Text::new("=== KAMPF LOG ===\nDrücke ENTER um den Kampf zu starten..."),
                TextFont {
                    font,
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 1.0, 0.6)),
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
        
        // Update text with color based on health
        for mut text in text_query.iter_mut() {
            **text = format!("{} / {}", health.current, health.max);
        }
    }
}

// Update room info
pub fn update_room_info(
    world: Res<WorldState>,
    mut query: Query<&mut Text, With<RoomInfoText>>,
) {
    for mut text in query.iter_mut() {
        **text = format!(
            "📍 {} - Raum {} von {}",
            world.current_room().name(),
            world.current_room_index + 1,
            world.total_rooms
        );
    }
}

// Update player stats display
pub fn update_player_stats(
    player_query: Query<(&Attack, &Defense, &Damage, &Armor), With<Player>>,
    mut query: Query<&mut Text, With<PlayerStatsText>>,
) {
    if let Ok((attack, defense, damage, armor)) = player_query.single() {
        for mut text in query.iter_mut() {
            **text = format!(
                "⚔️ Angriff: {} | 🛡️ Verteidigung: {} | 💥 Schaden: {} | 🔰 Rüstung: {}",
                attack.0, defense.0, damage.0, armor.0
            );
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
    if !combat_log.messages.is_empty() {
        for mut text in query.iter_mut() {
            let log_text = format!(
                "=== KAMPF LOG ===\n{}",
                combat_log.messages.join("\n")
            );
            **text = log_text;
        }
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
            let mut display_text: String = story.full_text.chars()
                .take(story.visible_chars)
                .collect();
            
            // Add blinking cursor if not finished
            if story.visible_chars < story.full_text.len() {
                display_text.push('▋');
            }
            
            **text = display_text;
        }
    }
}