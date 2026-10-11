use bevy::prelude::*;
use ron;
use serde::Deserialize;
use std::collections::HashMap;

pub struct MachinesPlugin;

impl Plugin for MachinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (setup_machine, (setup_resources, check_machines).chain()),
        );
    }
}

// ---

pub type ItemID = u16;

#[derive(Deserialize)]
struct RonItem {
    id: u16,
    name_id: u16, // must be defined in resources/names.ron (conventionally starting at 0x0)
    stack_size: u16,
}

#[derive(Deserialize)]
pub struct RonRecipe {
    id: u16,
    name_id: u16, // must be defined in resources/names.ron (conventionally starting at 0xF00)
    inputs: Vec<ItemID>,
    outputs: Vec<ItemID>,
    // Vec<ItemID> instead of an array, because it kept throwing deserialization errors
    // It gets converted to an array for better cache locality
}

#[derive(Deserialize, Debug)]
pub struct Item {
    name_id: u16, // must be defined in resources/locale.ron
    stack_size: u16,
}

#[derive(Debug)]
pub struct Recipe {
    name_id: u16,
    inputs: [ItemID; 6],
    outputs: [ItemID; 2],
}

#[derive(Deserialize)]
struct Name {
    id: u16,
    name: String,
}

#[derive(Resource)]
pub struct Items(HashMap<u16, Item>);

#[derive(Resource)]
pub struct Recipes(HashMap<u16, Recipe>);

#[derive(Resource)]
pub struct Names(HashMap<u16, String>);

// ########################
// ## Machine Components ##
// ########################

#[derive(Component)]
struct Machine;

#[derive(Component)]
struct RecipeID(u16);

#[derive(Component, Debug)]
struct Inventory {
    inputs: [u16; 6],
    outputs: [u16; 2],
}

struct NeedsUpdate;
struct Active;
struct Idle;
struct Unactive;

fn setup_resources(mut commands: Commands) {
    let items_content: String = match std::fs::read_to_string("resources/items.ron") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Could not read items from resources/items.ron");
            "".to_string()
        }
    };

    let recipe_content: String = match std::fs::read_to_string("resources/recipes.ron") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Could not read recipies from resources/recipes.ron");
            "".to_string()
        }
    };

    let names_content: String = match std::fs::read_to_string("resources/names.ron") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Could not read names from resources/names.ron");
            "".to_string()
        }
    };

    let deserialzed_items: Vec<RonItem> = match ron::from_str(&items_content) {
        Ok(ron) => ron,
        Err(e) => {
            eprintln!("Could not deserialize items.ron: {:#?}", e);
            vec![]
        }
    };

    let deserialzed_recipes: Vec<RonRecipe> = match ron::from_str(&recipe_content) {
        Ok(ron) => ron,
        Err(e) => {
            eprintln!("Could not deserialize recipes.ron: {:#?}", e);
            vec![]
        }
    };

    let deserialzed_names: Vec<Name> = match ron::from_str(&names_content) {
        Ok(ron) => ron,
        Err(e) => {
            eprintln!("Could not deserialize names.ron: {:#?}", e);
            vec![]
        }
    };

    let mut hash_items: HashMap<u16, Item> = HashMap::new();
    let mut hash_recipes: HashMap<u16, Recipe> = HashMap::new();
    let mut hash_names: HashMap<u16, String> = HashMap::new();

    for item in deserialzed_items {
        hash_items.insert(
            item.id,
            Item {
                name_id: item.name_id,
                stack_size: item.stack_size,
            },
        );
    }

    for recipe in deserialzed_recipes {
        let mut inputs: [ItemID; 6] = [0x0; 6];
        let mut outputs: [ItemID; 2] = [0x0; 2];

        for (i, input) in recipe.inputs.iter().enumerate() {
            inputs[i] = *input;
        }
        for (i, output) in recipe.outputs.iter().enumerate() {
            outputs[i] = *output;
        }

        hash_recipes.insert(
            recipe.id,
            Recipe {
                name_id: recipe.name_id,
                inputs: inputs,
                outputs: outputs,
            },
        );
    }

    for name in deserialzed_names {
        hash_names.insert(name.id, name.name);
    }

    dbg!(&hash_items);
    dbg!(&hash_recipes);
    dbg!(&hash_names);

    let items = Items(hash_items);
    let recipes = Recipes(hash_recipes);
    let names = Names(hash_names);

    commands.insert_resource(items);
    commands.insert_resource(recipes);
    commands.insert_resource(names);
}

fn setup_machine(mut commands: Commands) {
    commands.spawn((
        Machine,
        Inventory {
            inputs: [0x0; 6],
            outputs: [0x0; 2],
        },
        RecipeID(0x1),
    ));
}

fn check_machines(
    machines: Query<(&Inventory, &RecipeID), With<Machine>>,
    items: Res<Items>,
    recipes: Res<Recipes>,
    names: Res<Names>,
) {
    for machine in &machines {
        let (recipe_name, recipe_inputs, recipe_outputs): (String, [String; 6], [String; 2]) =
            if let Some(recipe) = recipes.0.get(&machine.1.0) {
                (
                    if let Some(name) = names.0.get(&recipe.name_id) {
                        name.clone()
                    } else {
                        "No Recipe".to_string()
                    },
                    recipe.inputs.map(|x| {
                        let name_id = if let Some(item) = items.0.get(&x) {
                            item.name_id
                        } else {
                            0x0
                        };
                        let name = if let Some(name) = names.0.get(&name_id) {
                            name.clone()
                        } else {
                            "".to_string()
                        };
                        name
                    }),
                    recipe.outputs.map(|x| {
                        let name_id = if let Some(item) = items.0.get(&x) {
                            item.name_id
                        } else {
                            0x0
                        };
                        let name = if let Some(name) = names.0.get(&name_id) {
                            name.clone()
                        } else {
                            "".to_string()
                        };
                        name
                    }),
                )
            } else {
                (
                    "No Recipe".to_string(),
                    [
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                    ],
                    ["".to_string(), "".to_string()],
                )
            };
        //dbg!(machine.0);
        println!(
            "Recipe: {recipe_name}\n  Inputs: {:?}\n  Outputs: {:?}",
            recipe_inputs, recipe_outputs
        );
    }
}
