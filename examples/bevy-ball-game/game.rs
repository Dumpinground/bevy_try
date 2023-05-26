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
            .add_system(resume_simulation.in_schedule(OnEnter(AppState::Game)))
            // System
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_plugin(GameUiPlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(pause_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
