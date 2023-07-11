use bevy::prelude::*;

use crate::fonts::FontSets;

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct Styles;

impl Styles {
    pub fn game_over_menu() -> Style {
        Style {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Style::DEFAULT
        }
    }

    pub fn game_over_menu_container() -> Style {
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Px(400.),
            height: Val::Px(400.),
            row_gap: Val::Px(8.),
            column_gap: Val::Px(8.),
            ..Style::DEFAULT
        }
    }

    pub fn button() -> Style {
        Style {
            width: Val::Px(200.),
            height: Val::Px(80.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Style::DEFAULT
        }
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(FontSets::edwardian()),
        font_size: 64.,
        color: Color::rgb(1., 1., 1.),
    }
}

pub fn get_final_score_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(FontSets::edwardian()),
        font_size: 48.,
        color: Color::rgb(1., 1., 1.),
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(FontSets::edwardian()),
        font_size: 32.,
        color: Color::rgb(1., 1., 1.),
    }
}
