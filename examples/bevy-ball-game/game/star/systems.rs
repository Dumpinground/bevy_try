use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::systems::random_position;

use super::components::Star;
use super::resources::*;
use super::NUMBER_OF_STARS;
use super::STAR_SIZE;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let (random_x, random_y) = random_position(window, STAR_SIZE);

        commands.spawn((
            Sprite::from_image(asset_server.load("ball-game/sprites/star.png")),
            Transform::from_xyz(random_x, random_y, 0.0),
            Star {},
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for star_entity in star_query.iter() {
        commands.entity(star_entity).despawn();
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let (random_x, random_y) = random_position(window, STAR_SIZE);

        commands.spawn((
            Sprite::from_image(asset_server.load("ball-game/sprites/star.png")),
            Transform::from_xyz(random_x, random_y, 0.0),
            Star {},
        ));
    }
}
