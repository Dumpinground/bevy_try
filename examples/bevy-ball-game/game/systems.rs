use bevy::prelude::*;
use rand::random;

use super::SimulationState;
use crate::AppState;

pub fn random_position(window: &Window, size: f32) -> (f32, f32) {
    let range = |length| {
        random::<f32>() * (length - size) + size / 2.0
    };
    (range(window.width()), range(window.height()))
}

pub fn pause_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    app_state: Res<State<AppState>>,
) {
    if app_state.eq(&AppState::Game) {
        simulation_state_next_state.set(SimulationState::Paused);
    }
}

pub fn resume_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    app_state: Res<State<AppState>>,
) {
    if app_state.eq(&AppState::Game) {
        simulation_state_next_state.set(SimulationState::Running);
    }
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    simulation_state_next_state: ResMut<NextState<SimulationState>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.eq(&SimulationState::Running) {
            pause_simulation(simulation_state_next_state, app_state);
            println!("Simulation Paused.");
        } else if simulation_state.eq(&SimulationState::Paused) {
            resume_simulation(simulation_state_next_state, app_state);
            println!("Simulation Running.");
        }
    }
}
