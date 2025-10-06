use bevy::prelude::*;

// enemytypes (wie EnemyType enum in java projekt)
#[derive(Clone, Copy, Debug)]
pub enum EnemyTypeData {
    MiniZombie,
    Scientist,
    BigZombie,
    Endboss,
}

impl EnemyTypeData {
    pub fn stats(&self) -> (u32, u32, u32, u32, u32, u32, u32, u32, u32, &'static str) {
        // life, armor, initiative, attack, defense, damage, numW6, finte, wuchtschlag, name
        match self {
            Self::MiniZombie => (10, 1, 12, 10, 6, 1, 1, 1, 0, "Mini Zombie"),
            Self::Scientist => (15, 0, 10, 12, 10, 4, 1, 2, 1, "Scientist"),
            Self::BigZombie => (20, 3, 6, 13, 8, 2, 2, 1, 2, "Big Zombie"),
            Self::Endboss => (50, 5, 4, 14, 10, 1, 4, 3, 3, "Endboss"),
        }
    }
}

// roomtypes (wie RoomType enum in java projekt)
#[derive(Clone, Copy, Debug)]
pub enum RoomTypeData {
    IntroRoom,
    FloorRoom,
    Pantry1,
    LibraryRoom,
    DiningHall,
    Laboratory,
    Corridor,
    FinalRoom,
}

impl RoomTypeData {
    pub fn enemies(&self) -> Vec<EnemyTypeData> {
        use EnemyTypeData::*;
        match self {
            Self::IntroRoom => vec![MiniZombie],
            Self::FloorRoom => vec![MiniZombie, MiniZombie],
            Self::Pantry1 => vec![MiniZombie, MiniZombie],
            Self::LibraryRoom => vec![MiniZombie, MiniZombie, Scientist],
            Self::DiningHall => vec![MiniZombie, MiniZombie, MiniZombie, Scientist, Scientist],
            Self::Laboratory => vec![Scientist, Scientist, Scientist, Scientist],
            Self::Corridor => vec![BigZombie, BigZombie, BigZombie],
            Self::FinalRoom => vec![Endboss],
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::IntroRoom => "Intro Room",
            Self::FloorRoom => "Floor Room",
            Self::Pantry1 => "Pantry",
            Self::LibraryRoom => "Library",
            Self::DiningHall => "Dining Hall",
            Self::Laboratory => "Laboratory",
            Self::Corridor => "Corridor",
            Self::FinalRoom => "Final Room",
        }
    }
}

// upgradetypes (wie UpgradeType enum in java projekt)
#[derive(Clone, Copy, Debug)]
pub enum UpgradeTypeData {
    Finte,
    Life,
    Damage,
    Skill,
    Armour,
    Attack,
    PantryCompound,
}

impl UpgradeTypeData {
    pub fn for_room(room: RoomTypeData) -> Option<Self> {
        match room {
            RoomTypeData::IntroRoom => Some(Self::Life),
            RoomTypeData::FloorRoom => Some(Self::Damage),
            RoomTypeData::Pantry1 => Some(Self::PantryCompound),
            RoomTypeData::LibraryRoom => Some(Self::Skill),
            RoomTypeData::DiningHall => Some(Self::Armour),
            RoomTypeData::Laboratory => Some(Self::Attack),
            RoomTypeData::Corridor => Some(Self::Finte),
            RoomTypeData::FinalRoom => None,
        }
    }
    
    // Returns (life, maxLife, armor, init, atk, def, dmg, finte, wucht)
    pub fn stats(&self) -> (u32, u32, u32, u32, u32, u32, u32, u32, u32) {
        match self {
            Self::Finte => (0, 0, 0, 0, 0, 0, 0, 1, 0),
            Self::Life => (5, 5, 0, 0, 0, 0, 0, 0, 0),
            Self::Damage => (0, 0, 0, 0, 0, 0, 2, 0, 0),
            Self::Skill => (0, 0, 0, 0, 0, 0, 0, 1, 1),
            Self::Armour => (0, 0, 3, 0, 0, 0, 0, 0, 0),
            Self::Attack => (0, 0, 0, 0, 2, 0, 0, 0, 0),
            Self::PantryCompound => (5, 5, 0, 0, 0, 0, 2, 0, 0),
        }
    }
}