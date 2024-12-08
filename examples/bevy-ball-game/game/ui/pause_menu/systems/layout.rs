use bevy::prelude::*;

use super::super::{components::*, styles::*};

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning Pause Menu");
    build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((Styles::pause_menu(), ZIndex(1), PauseMenu {}))
        .with_children(|parent| {
            parent
                .spawn((
                    Styles::pause_menu_container(),
                    BackgroundColor(BACKGROUND_COLOR),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Pause Menu"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        get_title_text_font(asset_server),
                        get_text_color(),
                    ));
                    // Resume Button
                    parent
                        .spawn((
                            Styles::button(),
                            Button,
                            BackgroundColor(NORMAL_BUTTON),
                            ResumeButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Resume"),
                                TextLayout::new_with_justify(JustifyText::Center),
                                get_button_text_font(asset_server),
                                get_text_color(),
                            ));
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            Styles::button(),
                            Button,
                            BackgroundColor(BACKGROUND_COLOR),
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Main Menu"),
                                TextLayout::new_with_justify(JustifyText::Center),
                                get_button_text_font(asset_server),
                                get_text_color(),
                            ));
                        });
                    // Quit Button
                    parent
                        .spawn((
                            Styles::button(),
                            Button,
                            BackgroundColor(BACKGROUND_COLOR),
                            QuitButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Quit"),
                                TextLayout::new_with_justify(JustifyText::Center),
                                get_button_text_font(asset_server),
                                get_text_color(),
                            ));
                        });
                });
        })
        .id();

    pause_menu_entity
}
