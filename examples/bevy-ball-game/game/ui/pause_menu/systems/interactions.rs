use crate::{game::SimulationState, AppState};
use bevy::{app::AppExit, prelude::*};

use super::super::{components::*, styles::*};

pub fn interact_with_resume_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                simulation_state_next_state.set(SimulationState::Running);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                app_state_next_state.set(AppState::MainMenu);
                simulation_state_next_state.set(SimulationState::Running);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                app_exit_event_writer.send(AppExit::Success);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
