mod events;
mod systems;

pub mod enemy;
mod player;
pub mod score;
pub mod star;

use events::*;
use systems::*;

use crate::player::PlayerPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugin(PlayerPlugin)
        .run();
}
