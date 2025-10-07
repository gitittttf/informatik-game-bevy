use bevy::prelude::*;

pub mod data;
pub mod resources;
pub mod systems;

pub use data::*;
pub use resources::*;
pub use systems::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<WorldState>();
}