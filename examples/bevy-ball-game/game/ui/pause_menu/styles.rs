use bevy::prelude::*;

use crate::fonts::FontSets;

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);

pub const PAUSE_MENU_STYLE: Style = Style {
    position_type: PositionType::Absolute,
    display: Display::Flex,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Px(100.0),
    height: Val::Px(100.0),
    ..Style::DEFAULT
};

pub const PAUSE_MENU_CONTAINER_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Px(400.0),
    height: Val::Px(400.0),
    row_gap: Val::Px(8.0),
    column_gap: Val::Px(8.0),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style {
    width: Val::Px(200.0),
    height: Val::Px(80.0),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..Style::DEFAULT
};

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(FontSets::edwardian()),
        font_size: 32.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(FontSets::edwardian()),
        font_size: 32.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}
