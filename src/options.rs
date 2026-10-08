// options.rs
//
// getting all options and configs from the config files
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        let game_options = match get_game_options() {
            Ok(options) => options,
            Err(e) => {
                eprintln!("Error: {}", e);
                GameOptions::default()
            }
        };

        app.insert_resource(game_options);
        app.add_systems(Startup, setup_game_options);
    }
}

#[derive(Deserialize, Serialize, Debug, Resource)]
pub struct GameOptions {
    pub fixed_tick_rate: f64,
}

impl Default for GameOptions {
    fn default() -> GameOptions {
        GameOptions {
            fixed_tick_rate: 1.0,
        }
    }
}

fn get_game_options() -> Result<GameOptions, Box<dyn std::error::Error>> {
    let mut options_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    options_path += "/options.ron";

    let contents: String = std::fs::read_to_string(options_path)?;

    let options: GameOptions = ron::from_str(&contents)?;

    Ok(options)
}

fn setup_game_options(mut commands: Commands, options: Res<GameOptions>) {
    commands.insert_resource(Time::<Fixed>::from_hz(options.fixed_tick_rate));
    println!("\x1b[33m{:?}\x1b[0m", options);
}
