use bevy::{camera::CameraPlugin, input::InputPlugin, prelude::*};

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
            .add_plugins(InputPlugin)
            .add_plugins(CameraPlugin)
            ;
    }
}