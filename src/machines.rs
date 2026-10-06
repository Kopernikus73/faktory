use bevy::prelude::*;

pub struct MachinesPlugin;

impl Plugin for MachinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_machine);
        app.add_systems(Update, show_machines);
    }
}

#[derive(Component)]
struct Machine;

#[derive(Component)]
struct Recipes {
    recipes: Vec<String>,
}

fn spawn_machine(mut commands: Commands) {
    commands.spawn((
        Machine,
        Recipes {
            recipes: vec!["Fish".to_string()],
        },
    ));
}

fn show_machines(query: Query<&Recipes, With<Machine>>) {
    for machine in &query {
        println!("Possible Recipes: {:?}", machine.recipes);
    }
}
