use bevy::{camera::CameraPlugin, input::InputPlugin as BevyInputPlugin, prelude::*};

mod game_state;

// import plugin modules
mod prelude;
mod input;
mod camera;
mod combat;
mod character;
mod world;
mod ui;

pub use game_state::GameState;
// Re-export the crate-local InputPlugin so binaries can use `informatik_game_bevy::InputPlugin`
pub use input::InputPlugin;

// Main plugin that ties everything together
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            //plugin registrations TODO
            .add_plugins(combat::plugin)
            .add_plugins(character::plugin)
            .add_plugins(ui::plugin)
            .add_plugins(world::plugin)
            .add_plugins(BevyInputPlugin)
            .add_plugins(CameraPlugin)
            ;
    }
}