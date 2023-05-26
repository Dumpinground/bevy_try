use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

// pub const FONT_PATH: &str = "ball-game/fonts/FiraSans-Bold.ttf";
pub const FONT_PATH: &str = "ball-game/fonts/JOKERMAN.TTF";

pub struct FontSets;

impl FontSets {
    fn jokerman() -> &'static str {
        "ball-game/fonts/JOKERMAN.TTF"
    }

    fn edwardian() -> &'static str {
        "ball-game/fonts/ITCEDSCR.TTF"
    }

    fn curlz() -> &'static str {
        "ball-game/fonts/CURLZ___.TTF"
    }
}

pub const MAIN_MENU_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const TITLE_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(300.0), Val::Px(120.0)),
    ..Style::DEFAULT
};

pub const IMAGE_STYLE: Style = Style {
    size: Size::new(Val::Px(64.0), Val::Px(64.0)),
    margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

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
