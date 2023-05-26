mod game_over_menu;
mod hud;
mod pause_menu;

use bevy::prelude::*;

use self::{hud::HudPlugin, game_over_menu::GameOverMenuPlugin};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HudPlugin)
        .add_plugin(GameOverMenuPlugin)
    }
}
