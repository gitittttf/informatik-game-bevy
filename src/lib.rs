use bevy::prelude::*;

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
pub use input::InputPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_systems(Startup, setup_camera) // Kamerasetup hinzufügen
            .add_plugins(combat::plugin)
            .add_plugins(character::plugin)
            .add_plugins(ui::plugin)
            .add_plugins(world::plugin);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}