use bevy::prelude::*;
use std::fmt;

mod machines;
mod options;

// Entities
#[derive(Component)]
struct Player;

// Components

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

// Inventory

#[derive(Copy, Clone, Debug)]
enum Item {
    None,
    IronPlate,
    CopperPlate,
    Coal,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::None => write!(f, "_"),
            Item::IronPlate => write!(f, "Iron Plate"),
            Item::CopperPlate => write!(f, "Copper Plate"),
            Item::Coal => write!(f, "Coal"),
        }
    }
}

#[derive(Copy, Clone)]
struct InventoryItem {
    item: Item,
    stack: u8,
}

impl fmt::Display for InventoryItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.stack, self.item)
    }
}

#[derive(Component)]
struct Inventory {
    items: [InventoryItem; 16],
}

impl Default for Inventory {
    fn default() -> Inventory {
        Inventory {
            items: [InventoryItem {
                item: Item::None,
                stack: 0,
            }; 16],
        }
    }
}

// Systems
fn setup_player(mut commands: Commands) {
    let mut example_player_inv = Inventory::default();
    example_player_inv.items[0] = InventoryItem {
        item: Item::CopperPlate,
        stack: 1,
    };
    example_player_inv.items[1] = InventoryItem {
        item: Item::IronPlate,
        stack: 32,
    };
    example_player_inv.items[2] = InventoryItem {
        item: Item::Coal,
        stack: 4,
    };

    commands.spawn((Player, Position { x: 123.0, y: 321.0 }, example_player_inv));
}

/*
fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("P({}|{})", position.x, position.y);
    }
}

fn print_inventory(query: Query<&Inventory, With<Player>>) {
    println!("Inventory: ");
    for inv in &query {
        for (id, item) in inv.items.iter().enumerate() {
            print!("{}", item);
            if (id + 1) % 8 == 0 {
                println!();
            }
        }
    }
}
*/

// Main App
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(options::OptionsPlugin)
        //.add_plugins(machines::MachinesPlugin)
        .add_systems(Startup, setup_player)
        //.add_systems(Update, )
        //.add_systems(FixedUpdate, )
        .run();
}
