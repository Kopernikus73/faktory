use bevy::prelude::*;
//use bevy_ui;
use std::fmt;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_player, setup_camera));
        app.add_systems(Update, (update_position_text, move_player, update_camera));
    }
}

const PLAYER_SPEED: f32 = 10.0;

// ############################
// ## Main Player Components ##
// ############################

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PostionTextMarker;

// ###############
// ## Inventory ##
// ###############

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
    stack: u16,
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

impl Inventory {
    fn insert(mut self, id: usize, item: Item, stack: u16) -> Inventory {
        if id < self.items.len() {
            self.items[id] = InventoryItem {
                item: item,
                stack: stack,
            }
        }
        self
    }
}

// ## Systems ##

// setup

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let example_player_inv = Inventory::default()
        .insert(0, Item::CopperPlate, 1)
        .insert(1, Item::IronPlate, 10)
        .insert(2, Item::Coal, 34);

    commands.spawn((
        Player,
        Transform::default(),
        Sprite::from_image(asset_server.load("player.png")),
        example_player_inv,
    ));
}

// Camera

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::default(),
        Projection::from(OrthographicProjection {
            scale: 0.125,
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Text::new("Use WASD to move"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            right: px(12),
            ..default()
        },
    ));

    commands.spawn((Text::new("You're at Pos: (0,0)"), PostionTextMarker));
}

// Position

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    camera.translation = player.translation;
}

fn update_position_text(
    player_position: Single<&Transform, (Changed<Transform>, With<Player>)>,
    mut text_query: Single<&mut Text, With<PostionTextMarker>>,
) {
    text_query.0 = format!(
        "You're at Pos: ({:.2},{:.2})",
        (player_position.translation.x * 100.).round() / 100.,
        (player_position.translation.y * 100.).round() / 100.
    );
}

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction[1] += 1.;
    };

    if kb_input.pressed(KeyCode::KeyS) {
        direction[1] -= 1.;
    };

    if kb_input.pressed(KeyCode::KeyD) {
        direction[0] += 1.;
    };

    if kb_input.pressed(KeyCode::KeyA) {
        direction[0] -= 1.;
    };

    let delta_direction = direction.normalize_or_zero() * time.delta_secs() * PLAYER_SPEED;
    player.translation += delta_direction.extend(0.);
}
