use bevy::prelude::*;
use crate::character::*;
use crate::combat::*;
use super::CombatLog;

#[derive(Component)]
pub struct CombatUIMarker;

#[derive(Component)]
pub struct PlayerHPBar;

#[derive(Component)]
pub struct PlayerHPText;

#[derive(Component)]
pub struct EnemyListText;

#[derive(Component)]
pub struct SelectedEnemyInfo;

#[derive(Component)]
pub struct CombatInstructionText;

#[derive(Component)]
pub struct CombatLogDisplay;

#[derive(Resource, Default)]
pub struct CombatUIState {
    pub selected_enemy_index: usize,
    pub selected_finte: u32,
    pub selected_wuchtschlag: u32,
    pub input_phase: CombatInputPhase,
}

#[derive(Default, PartialEq)]
pub enum CombatInputPhase {
    #[default]
    SelectingEnemy,
    SelectingFinte,
    SelectingWuchtschlag,
}

pub fn setup_combat_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/atlantisheadbold.ttf");
    
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgb(0.02, 0.02, 0.05)),
        CombatUIMarker,
    ))
    .with_children(|parent| {
        // Top bar - Status
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.0, 0.0)),
            BorderColor::all(Color::srgb(0.8, 0.0, 0.0)),
        ))
        .with_child((
            Text::new("⚔️ KAMPF ⚔️"),
            TextFont {
                font: font.clone(),
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.2, 0.2)),
        ));
        
        // Main content area (3 columns)
        parent.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .with_children(|main| {
            // Left column: Player info
            create_player_column(main, &font);
            
            // Middle column: Combat scene
            create_combat_scene_column(main, &font);
            
            // Right column: Enemy info
            create_enemy_column(main, &font);
        });
        
        // Bottom: Combat log
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(200.0),
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::top(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 0.05, 0.0)),
            BorderColor::all(Color::srgb(0.0, 0.5, 0.0)),
        ))
        .with_child((
            Text::new("=== KAMPF-LOG ===\n"),
            TextFont {
                font: font.clone(),
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 1.0, 0.6)),
            CombatLogDisplay,
        ));
    });
}

fn create_player_column(parent: &mut ChildSpawnerCommands<'_>, font: &Handle<Font>) {
    parent.spawn((
        Node {
            width: Val::Percent(33.3),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            border: UiRect::right(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.05, 0.1)),
        BorderColor::all(Color::srgb(0.3, 0.3, 0.4)),
    ))
    .with_children(|col| {
        // Title
        col.spawn((
            Text::new("[ SPIELER ]"),
            TextFont {
                font: font.clone(),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 1.0)),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
        
        // HP Bar container
        col.spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        })
        .with_children(|hp_container| {
            hp_container.spawn((
                Text::new("♥ HP"),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.5, 0.5)),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));
            
            // HP bar background
            hp_container.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.0, 0.0)),
                BorderColor::all(Color::srgb(0.6, 0.2, 0.2)),
            ))
            .with_child((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                PlayerHPBar,
            ));
            
            hp_container.spawn((
                Text::new("100 / 100"),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
                PlayerHPText,
            ));
        });
        
        // Combat instructions
        col.spawn((
            Text::new("Wähle Aktion..."),
            TextFont {
                font: font.clone(),
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.5)),
            CombatInstructionText,
        ));
    });
}

fn create_combat_scene_column(parent: &mut ChildSpawnerCommands<'_>, font: &Handle<Font>) {
    parent.spawn((
        Node {
            width: Val::Percent(33.3),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::right(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.03, 0.03, 0.06)),
        BorderColor::all(Color::srgb(0.3, 0.3, 0.4)),
    ))
    .with_children(|col| {
        col.spawn((
            Text::new("⚔️  ⚡  💥"),
            TextFont {
                font: font.clone(),
                font_size: 64.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.8, 0.0)),
        ));
    });
}

fn create_enemy_column(parent: &mut ChildSpawnerCommands<'_>, font: &Handle<Font>) {
    parent.spawn((
        Node {
            width: Val::Percent(33.3),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.05, 0.05)),
    ))
    .with_children(|col| {
        // Title
        col.spawn((
            Text::new("[ GEGNER ]"),
            TextFont {
                font: font.clone(),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.3, 0.3)),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
        
        // Selected enemy details
        col.spawn((
            Text::new("Wähle einen Gegner..."),
            TextFont {
                font: font.clone(),
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
            SelectedEnemyInfo,
        ));
        
        // Enemy list
        col.spawn((
            Text::new("Keine Gegner"),
            TextFont {
                font: font.clone(),
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            EnemyListText,
        ));
    });
}

// Update systems
pub fn update_combat_ui(
    ui_state: Res<CombatUIState>,
    player_query: Query<(&Health, &SpecialAbilities), With<Player>>,
    enemy_query: Query<(Entity, &CharacterType, &Health), With<Enemy>>,
    mut hp_bar_query: Query<&mut Node, With<PlayerHPBar>>,
    mut hp_text_query: Query<&mut Text, With<PlayerHPText>>,
    mut instruction_query: Query<&mut Text, (With<CombatInstructionText>, Without<PlayerHPText>, Without<EnemyListText>, Without<SelectedEnemyInfo>)>,
    mut enemy_list_query: Query<&mut Text, (With<EnemyListText>, Without<CombatInstructionText>, Without<PlayerHPText>, Without<SelectedEnemyInfo>)>,
    mut selected_enemy_query: Query<&mut Text, (With<SelectedEnemyInfo>, Without<CombatInstructionText>, Without<PlayerHPText>, Without<EnemyListText>)>,
) {
    // Update player HP
    if let Ok((health, abilities)) = player_query.single() {
        let percent = (health.current as f32 / health.max as f32) * 100.0;
        
        for mut node in hp_bar_query.iter_mut() {
            node.width = Val::Percent(percent);
        }
        
        for mut text in hp_text_query.iter_mut() {
            **text = format!("{} / {}", health.current, health.max);
        }
        
        // Update instructions based on phase
        for mut text in instruction_query.iter_mut() {
            **text = match ui_state.input_phase {
                CombatInputPhase::SelectingEnemy => {
                    format!(
                        "Wähle Gegner:\n← → oder 1-{}\n\nENTER: Weiter",
                        enemy_query.iter().count()
                    )
                }
                CombatInputPhase::SelectingFinte => {
                    format!(
                        "Finte Level:\n← → oder 0-{}\n\nAktuell: {}\n\nENTER: Weiter\nESC: Zurück",
                        abilities.finte_level,
                        ui_state.selected_finte
                    )
                }
                CombatInputPhase::SelectingWuchtschlag => {
                    format!(
                        "Wuchtschlag Level:\n← → oder 0-{}\n\nAktuell: {}\n\nENTER: Angriff!\nESC: Zurück",
                        abilities.wuchtschlag_level,
                        ui_state.selected_wuchtschlag
                    )
                }
            };
        }
    }
    
    // Update enemy list
    let enemies: Vec<_> = enemy_query.iter().collect();
    if !enemies.is_empty() {
        // Update enemy list
        for mut text in enemy_list_query.iter_mut() {
            let mut list = String::from("Alle Gegner:\n\n");
            for (i, (_, char_type, health)) in enemies.iter().enumerate() {
                let marker = if i == ui_state.selected_enemy_index { "►" } else { " " };
                list.push_str(&format!(
                    "{} {}. {} ({} HP)\n",
                    marker,
                    i + 1,
                    char_type.0,
                    health.current
                ));
            }
            **text = list;
        }
        
        // Update selected enemy details
        if let Some((_, char_type, health)) = enemies.get(ui_state.selected_enemy_index) {
            for mut text in selected_enemy_query.iter_mut() {
                **text = format!(
                    "Aktuelles Ziel:\n\n{}\n\nHP: {} / {}",
                    char_type.0,
                    health.current,
                    health.max
                );
            }
        }
    }
}

pub fn update_combat_log_display(
    combat_log: Res<CombatLog>,
    mut query: Query<&mut Text, With<CombatLogDisplay>>,
) {
    for mut text in query.iter_mut() {
        if !combat_log.messages.is_empty() {
            **text = format!(
                "=== KAMPF-LOG ===\n{}",
                combat_log.messages.join("\n")
            );
        }
    }
}

// Input handling
pub fn handle_combat_ui_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<CombatUIState>,
    mut commands: Commands,
    player_query: Query<(Entity, &SpecialAbilities), With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    battle_state: Res<BattleState>,
) {
    // Only allow input when it's the player's turn
    if !battle_state.combat_active || !battle_state.waiting_for_player_input {
        return;
    }
    
    let enemy_count = enemy_query.iter().count();
    if enemy_count == 0 {
        return;
    }
    
    let Ok((player_entity, abilities)) = player_query.single() else {
        return;
    };
    
    match ui_state.input_phase {
        CombatInputPhase::SelectingEnemy => {
            if keyboard.just_pressed(KeyCode::ArrowLeft) && ui_state.selected_enemy_index > 0 {
                ui_state.selected_enemy_index -= 1;
            } else if keyboard.just_pressed(KeyCode::ArrowRight) && ui_state.selected_enemy_index < enemy_count - 1 {
                ui_state.selected_enemy_index += 1;
            } else if keyboard.just_pressed(KeyCode::Digit1) && enemy_count >= 1 {
                ui_state.selected_enemy_index = 0;
            } else if keyboard.just_pressed(KeyCode::Digit2) && enemy_count >= 2 {
                ui_state.selected_enemy_index = 1;
            } else if keyboard.just_pressed(KeyCode::Digit3) && enemy_count >= 3 {
                ui_state.selected_enemy_index = 2;
            } else if keyboard.just_pressed(KeyCode::Digit4) && enemy_count >= 4 {
                ui_state.selected_enemy_index = 3;
            } else if keyboard.just_pressed(KeyCode::Digit5) && enemy_count >= 5 {
                ui_state.selected_enemy_index = 4;
            } else if keyboard.just_pressed(KeyCode::Enter) {
                ui_state.input_phase = CombatInputPhase::SelectingFinte;
                ui_state.selected_finte = 0;
            }
        }
        
        CombatInputPhase::SelectingFinte => {
            if keyboard.just_pressed(KeyCode::ArrowLeft) && ui_state.selected_finte > 0 {
                ui_state.selected_finte -= 1;
            } else if keyboard.just_pressed(KeyCode::ArrowRight) && ui_state.selected_finte < abilities.finte_level {
                ui_state.selected_finte += 1;
            } else if keyboard.just_pressed(KeyCode::Digit0) {
                ui_state.selected_finte = 0;
            } else if keyboard.just_pressed(KeyCode::Digit1) && abilities.finte_level >= 1 {
                ui_state.selected_finte = 1;
            } else if keyboard.just_pressed(KeyCode::Digit2) && abilities.finte_level >= 2 {
                ui_state.selected_finte = 2;
            } else if keyboard.just_pressed(KeyCode::Digit3) && abilities.finte_level >= 3 {
                ui_state.selected_finte = 3;
            } else if keyboard.just_pressed(KeyCode::Enter) {
                ui_state.input_phase = CombatInputPhase::SelectingWuchtschlag;
                ui_state.selected_wuchtschlag = 0;
            } else if keyboard.just_pressed(KeyCode::Escape) {
                ui_state.input_phase = CombatInputPhase::SelectingEnemy;
            }
        }
        
        CombatInputPhase::SelectingWuchtschlag => {
            if keyboard.just_pressed(KeyCode::ArrowLeft) && ui_state.selected_wuchtschlag > 0 {
                ui_state.selected_wuchtschlag -= 1;
            } else if keyboard.just_pressed(KeyCode::ArrowRight) && ui_state.selected_wuchtschlag < abilities.wuchtschlag_level {
                ui_state.selected_wuchtschlag += 1;
            } else if keyboard.just_pressed(KeyCode::Digit0) {
                ui_state.selected_wuchtschlag = 0;
            } else if keyboard.just_pressed(KeyCode::Digit1) && abilities.wuchtschlag_level >= 1 {
                ui_state.selected_wuchtschlag = 1;
            } else if keyboard.just_pressed(KeyCode::Digit2) && abilities.wuchtschlag_level >= 2 {
                ui_state.selected_wuchtschlag = 2;
            } else if keyboard.just_pressed(KeyCode::Digit3) && abilities.wuchtschlag_level >= 3 {
                ui_state.selected_wuchtschlag = 3;
            } else if keyboard.just_pressed(KeyCode::Enter) {
                // Execute attack!
                let enemies: Vec<Entity> = enemy_query.iter().collect();
                if let Some(&target) = enemies.get(ui_state.selected_enemy_index) {
                    commands.entity(player_entity).insert(QueuedAction {
                        target: Some(target),
                        finte_level: ui_state.selected_finte,
                        wuchtschlag_level: ui_state.selected_wuchtschlag,
                    });
                }
                
                // Reset to selecting enemy
                ui_state.input_phase = CombatInputPhase::SelectingEnemy;
                ui_state.selected_finte = 0;
                ui_state.selected_wuchtschlag = 0;
            } else if keyboard.just_pressed(KeyCode::Escape) {
                ui_state.input_phase = CombatInputPhase::SelectingFinte;
            }
        }
    }
}