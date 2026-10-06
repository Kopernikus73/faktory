use bevy::prelude::*;

mod machines;

// Entities
#[derive(Component)]
struct Player;

// Components

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

// Systems
fn setup(mut commands: Commands) {
    commands.spawn((Player, Position { x: 123.0, y: 321.0 }));
}

fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("P({}|{})", position.x, position.y);
    }
}

// Main App
fn main() {
    App::new()
        .add_plugins(machines::MachinesPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, print_position_system)
        .run();
}
