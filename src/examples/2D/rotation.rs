use bevy::{
    prelude::*,
};
use bevy::math::Vec3Swizzles;

const TIME_STEP: f32 = 1.0 / 165.0; // 显示器刷新
const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_systems(
            (
                player_movement_system,
                snap_to_player_system,
                rotate_to_player_system,
            )
        )
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

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    transform.rotate_z(rotation_factor * ship.rotation_speed * TIME_STEP);

    let movement_direction = transform.rotation * Vec3::Y;
    let movement_distance = movement_factor * ship.movement_speed * TIME_STEP;
    let translation_delta = movement_direction * movement_distance;
    transform.translation += translation_delta;

    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}

fn snap_to_player_system(
    mut query: Query<&mut Transform, (With<SnapToPlayer>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.xy();

    for mut enemy_transform in &mut query {
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
    }
}

fn rotate_to_player_system() {}
