use bevy::{
    DefaultPlugins,
    app::{App, Startup},
};

mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, state::initialize_game_state)
        .run();
}

// TODO after initializing the game state, need to read in the game state
// in order to setup the ui.
