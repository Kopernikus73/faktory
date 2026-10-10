use bevy::prelude::*;

mod machines;
mod options;
mod player;

// Main App
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(options::OptionsPlugin)
        .add_plugins(player::PlayerPlugin)
        //.add_plugins(machines::MachinesPlugin)
        .run();
}
