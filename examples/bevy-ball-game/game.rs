pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;
mod ui;

use crate::{events::GameOver, AppState};
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use systems::{pause_simulation, resume_simulation, toggle_simulation};

use self::ui::GameUiPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Event
            .add_event::<GameOver>()
            .add_state::<SimulationState>()
            .add_systems(OnEnter(AppState::Game), resume_simulation)
            // System
            .add_plugins((
                PlayerPlugin,
                EnemyPlugin,
                ScorePlugin,
                StarPlugin,
                GameUiPlugin,
            ))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), pause_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
