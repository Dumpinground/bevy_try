use bevy::{prelude::*, a11y::accesskit::TextAlign};

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const GAME_OVER_MENU_STYLE: Style;

pub const GAME_OVER_MENU_CONTAINER_STYLE: Style;

pub const BUTTON_STYLE: Style;

pub const FONT_PATH: &str = "ball/game/fonts/JOKERMAN.TTF";

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle { font: asset_server.load(FONT_PATH), font_size: 64.0, color: Color::rgb(1.0, 1.0, 1.0) }
}

pub fn get_final_score_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle { font: asset_server.load(FONT_PATH), font_size: 48.0, color: Color::rgb(1.0, 1.0, 1.0) }
}
