pub mod enemy;
mod player;
pub mod score;
pub mod star;

use crate::events::GameOver;
use bevy::app::{App, Plugin};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Event
            .add_event::<GameOver>()
            // System
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin);
    }
}
