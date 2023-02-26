use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::prelude::{App, Component, Res};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct SnapToPlayer;

fn setup(asset_server: Res<AssetServer>) {
    let ship_handle = asset_server.load("/textures/simplespace/ship_C.png");
}
