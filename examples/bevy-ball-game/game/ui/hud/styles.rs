use bevy::prelude::*;

use crate::fonts::FontSets;

pub const BACKGROUND_COLOR: Color = Color::srgba(0.25, 0.25, 0.25, 0.5);

pub struct Styles;

impl Styles {
    pub fn hud() -> Node {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(15.0),
            ..default()
        }
    }

    pub fn lhs() -> Node {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(200.0),
            height: Val::Percent(80.0),
            margin: UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
            ..default()
        }
    }

    pub fn rhs() -> Node {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(200.0),
            height: Val::Percent(80.0),
            margin: UiRect::new(Val::Px(0.0), Val::Px(32.0), Val::Px(0.0), Val::Px(0.0)),
            ..default()
        }
    }

    pub fn image() -> Node {
        Node {
            width: Val::Px(48.0),
            height: Val::Px(48.0),
            margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
            ..default()
        }
    }
}

pub fn get_text_font(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load(FontSets::jokerman()),
        font_size: 64.0,
        ..default()
    }
}

pub fn get_text_color() -> TextColor {
    TextColor(Color::srgb(1.0, 1.0, 1.0))
}
