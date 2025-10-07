pub use crate::game_state::GameState;

// Character exports
pub use crate::character::{
    bundles::*,
    components::*,
};

// Combat exports
pub use crate::combat::{
    components::*,
    events::*,
    resources::*,
    systems::*,
};

// World exports
pub use crate::world::{
    data::*,
    resources::*,
    systems::*,
};

// Re-export commonly used Bevy types
pub use bevy::prelude::*;