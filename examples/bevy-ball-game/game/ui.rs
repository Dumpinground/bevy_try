mod game_over_menu;
mod hud;
mod pause_menu;

use bevy::prelude::*;

use self::{game_over_menu::GameOverMenuPlugin, hud::HudPlugin, pause_menu::PauseMenuPlugin};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HudPlugin,
            PauseMenuPlugin, 
            GameOverMenuPlugin
        ));
    }
}
