use crate::fonts::FontSets;
use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct Styles;

impl Styles {
    pub fn main_menu() -> Style {
        Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            row_gap: Val::Px(8.0),
            column_gap: Val::Px(8.0),
            ..Style::DEFAULT
        }
    }

    pub fn title() -> Style {
        Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Auto,
            height: Val::Px(120.0),
            ..Style::DEFAULT
        }
    }

    pub fn image() -> Style {
        Style {
            width: Val::Px(64.),
            height: Val::Px(64.),
            margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
            ..Style::DEFAULT
        }
    }

    pub fn button() -> Style {
        Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Px(200.0),
            height: Val::Px(80.0),
            ..Style::DEFAULT
        }
    }
}

pub fn get_title_text_style(assert_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: assert_server.load(FontSets::jokerman()),
        font_size: 64.0,
        color: Color::WHITE,
    }
}

pub fn get_button_text_style(assert_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: assert_server.load(FontSets::curlz()),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
