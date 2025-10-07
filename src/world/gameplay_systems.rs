use bevy::prelude::*;
use crate::character::*;
use crate::game_state::GameState;
use crate::combat::*;
use crate::ui::StoryText;
use super::data::*;
use super::resources::*;

// System that spawns the player when entering gameplay
pub fn spawn_player_system(
    mut commands: Commands,
    char_selection: Res<crate::input::CharacterSelection>,
) {
    // Get player stats based on selection
    let (life, armor, init, attack, defense, damage, dice, finte, wucht) = 
        match char_selection.current {
            0 => (20, 1, 14, 12, 10, 3, 2, 1, 1), // Schwertkrieger
            1 => (25, 3, 10, 10, 12, 2, 2, 1, 0), // Schildkrieger
            _ => (20, 1, 14, 12, 10, 3, 2, 1, 1),
        };
    
    commands.spawn(PlayerBundle::new(
        life, armor, init, attack, defense, damage, dice, finte, wucht
    ));
    
    info!("Player spawned with {} HP", life);
}

// System that loads a room and displays its story
pub fn load_room_system(
    mut story_text: ResMut<StoryText>,
    world: Res<WorldState>,
) {
    let room = world.current_room();
    
    // Set story text based on room
    let story = match room {
        RoomTypeData::IntroRoom => {
            "Du erwachst in einem dunklen Raum. Die Luft ist feucht und riecht nach Verwesung. \
             Ein untotes Wesen kriecht auf dich zu..."
        }
        RoomTypeData::FloorRoom => {
            "Der Boden ist übersät mit Blut und Knochen. Zwei Zombies blockieren deinen Weg."
        }
        RoomTypeData::Pantry1 => {
            "Du betrittst eine alte Vorratskammer. Verweste Lebensmittel liegen überall herum. \
             Doch hier lauern auch Gefahren..."
        }
        RoomTypeData::LibraryRoom => {
            "Eine große Bibliothek voller verstaubter Bücher. Doch nicht alle hier sind tot..."
        }
        RoomTypeData::DiningHall => {
            "Der Speisesaal war einst prächtig. Jetzt ist er voller Untoten, die auf ihr nächstes Mahl warten."
        }
        RoomTypeData::Laboratory => {
            "Ein Labor voller mysteriöser Geräte. Die Wissenschaftler arbeiten noch immer... \
             oder das, was von ihnen übrig ist."
        }
        RoomTypeData::Corridor => {
            "Ein langer Korridor. Am Ende siehst du massive Gestalten auf dich zukommen."
        }
        RoomTypeData::FinalRoom => {
            "Der finale Raum. Hier lauert der Meister aller Untoten. Dies ist dein letzter Kampf!"
        }
    };
    
    story_text.full_text = story.to_string();
    story_text.visible_chars = 0;
    story_text.timer.reset();
    
    info!("Loaded room: {}", room.name());
}

// System that starts combat when story is finished
pub fn start_combat_when_ready_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    story_text: Res<StoryText>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut battle_state: ResMut<BattleState>,
    world: Res<WorldState>,
) {
    // Wait for story to finish and player to press Enter
    if story_text.visible_chars >= story_text.full_text.len() 
        && keyboard.just_pressed(KeyCode::Enter) 
    {
        // Spawn enemies for current room
        let room = world.current_room();
        for enemy_type in room.enemies() {
            let stats = enemy_type.stats();
            commands.spawn(EnemyBundle::new(
                stats.9.to_string(), stats.0, stats.1, stats.2, 
                stats.3, stats.4, stats.5, stats.6, stats.7, stats.8
            ));
        }
        
        // Start combat
        battle_state.combat_active = true;
        next_state.set(GameState::Combat);
        info!("Starting combat!");
    }
}

// System that handles combat end
pub fn handle_combat_end_system(
    mut commands: Commands,
    mut combat_end_events: MessageReader<CombatEndEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut world: ResMut<WorldState>,
    mut story_text: ResMut<StoryText>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for event in combat_end_events.read() {
        // Despawn all enemies
        for entity in enemy_query.iter() {
            commands.entity(entity).despawn();
        }
        
        if event.player_won {
            // Apply room upgrade
            if let Some(upgrade) = UpgradeTypeData::for_room(world.current_room()) {
                let upgrade_text = match upgrade {
                    UpgradeTypeData::Life => "Du findest ein Heilmittel! +5 HP",
                    UpgradeTypeData::Damage => "Du findest eine bessere Waffe! +2 Schaden",
                    UpgradeTypeData::Finte => "Du lernst eine neue Finten-Technik!",
                    UpgradeTypeData::Skill => "Du verbesserst deine Kampffähigkeiten!",
                    UpgradeTypeData::Armour => "Du findest bessere Rüstung! +3 Rüstung",
                    UpgradeTypeData::Attack => "Du trainierst deinen Angriff! +2 Angriff",
                    UpgradeTypeData::PantryCompound => "Du findest eine mächtige Substanz! +5 HP und +2 Schaden",
                };
                
                story_text.full_text = format!(
                    "Du hast alle Gegner besiegt!\n\n{}\n\nDrücke Enter für den nächsten Raum...",
                    upgrade_text
                );
            } else {
                story_text.full_text = "Du hast den Endboss besiegt! Du hast gewonnen!\n\nGlückwunsch!".to_string();
            }
            
            story_text.visible_chars = 0;
            story_text.timer.reset();
            
            // Check if there are more rooms
            if world.has_next_room() {
                world.advance();
                next_state.set(GameState::Gameplay);
            } else {
                // Game won!
                info!("Game completed!");
            }
        } else {
            // Player died
            story_text.full_text = "Du wurdest besiegt...\n\nDrücke ESC für das Hauptmenü".to_string();
            story_text.visible_chars = 0;
            next_state.set(GameState::Gameplay);
        }
    }
}

// Apply upgrades after combat
pub fn apply_upgrades_system(
    world: Res<WorldState>,
    mut player_query: Query<(
        &mut Health,
        &mut Armor,
        &mut Attack,
        &mut Defense,
        &mut Damage,
        &mut SpecialAbilities,
    ), With<Player>>,
) {
    if let Ok((mut health, mut armor, mut attack, mut defense, mut damage, mut abilities)) = player_query.single_mut() {
        let room_index = if world.current_room_index > 0 {
            world.current_room_index - 1
        } else {
            return;
        };
        
        let room = world.rooms.get(room_index);
        if let Some(&room) = room {
            if let Some(upgrade) = UpgradeTypeData::for_room(room) {
                let stats = upgrade.stats();
                health.current += stats.0;
                health.max += stats.1;
                armor.0 += stats.2;
                attack.0 += stats.4;
                defense.0 += stats.5;
                damage.0 += stats.6;
                abilities.finte_level += stats.7;
                abilities.wuchtschlag_level += stats.8;
                
                info!("Applied upgrade: {:?}", upgrade);
            }
        }
    }
}