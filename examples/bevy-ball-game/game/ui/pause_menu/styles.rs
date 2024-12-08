use bevy::prelude::*;

use crate::fonts::FontSets;

pub const BACKGROUND_COLOR: Color = Color::srgba(0.25, 0.25, 0.25, 0.5);

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

pub struct Styles;

impl Styles {
    pub fn pause_menu() -> Node {
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            // width: Val::Px(100.0),
            // height: Val::Px(100.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        }
    }

    pub fn pause_menu_container() -> Node {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Px(400.0),
            height: Val::Px(400.0),
            row_gap: Val::Px(8.0),
            column_gap: Val::Px(8.0),
            ..default()
        }
    }

    pub fn button() -> Node {
        Node {
            width: Val::Px(200.0),
            height: Val::Px(80.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }
}

pub fn get_title_text_font(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load(FontSets::edwardian()),
        font_size: 32.0,
        ..default()
    }
}

pub fn get_button_text_font(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load(FontSets::edwardian()),
        font_size: 32.0,
        ..default()
    }
}

pub fn get_text_color() -> TextColor {
    TextColor(Color::srgb(1., 1., 1.))
}
