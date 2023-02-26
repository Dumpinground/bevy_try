use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::prelude::*;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}

#[derive(Component)]
struct SnapToPlayer;

#[derive(Component)]
struct RotateToPlayer {
    rotation_speed: f32
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let asset_path = String::from("textures/simplespace/");
    let ship_handle = asset_server.load(asset_path.clone() + "ship_C.png");
    let enemy_a_handle = asset_server.load(asset_path.clone() + "enemy_A.png");
    let enemy_b_handle = asset_server.load(asset_path + "enemy_B.png");

    commands.spawn(Camera2dBundle::default());

    let horizontal_margin = BOUNDS.x / 4.0;
    let vertical_margin = BOUNDS.y / 4.0;

    commands.spawn((
        SpriteBundle {
            texture: ship_handle,
            ..default()
        }, 
        Player {
            movement_speed: 500.0,
            rotation_speed: f32::to_radians(360.0),
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: enemy_a_handle.clone(),
            transform: Transform::from_xyz(0.0 - horizontal_margin, 0.0, 0.0),
            ..default()
        },
        SnapToPlayer,
    ));
    commands.spawn((
        SpriteBundle {
            texture: enemy_a_handle,
            transform: Transform::from_xyz(0.0, 0.0 - vertical_margin, 0.0),
            ..default()
        },
        SnapToPlayer,
    ));

    commands.spawn((
        SpriteBundle {
            texture: enemy_b_handle.clone(),
            transform: Transform::from_xyz(0.0 + horizontal_margin, 0.0, 0.0),
            ..default()
        },
        RotateToPlayer {
            rotation_speed: f32::to_radians(45.0)
        }
    ));
    commands.spawn((
        SpriteBundle {
            texture: enemy_b_handle,
            transform: Transform::from_xyz(0.0, 0.0 + vertical_margin, 0.0),
            ..default()
        },
        RotateToPlayer {
            rotation_speed: f32::to_radians(90.0)
        }
    ));
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (ship, mut transform) = query.single_mut();
}