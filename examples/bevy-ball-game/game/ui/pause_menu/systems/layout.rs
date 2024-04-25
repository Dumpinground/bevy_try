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
        .spawn((
            NodeBundle {
                style: Styles::pause_menu(),
                z_index: ZIndex::Local(1),
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Styles::pause_menu_container(),
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Pause Menu",
                                get_title_text_style(asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Resume Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Styles::button(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            ResumeButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style::DEFAULT,
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Resume",
                                        get_button_text_style(asset_server),
                                    )],
                                    justify: JustifyText::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Styles::button(),
                                background_color: BACKGROUND_COLOR.into(),
                                ..default()
                            },
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style::DEFAULT,
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Main Menu",
                                        get_button_text_style(asset_server),
                                    )],
                                    justify: JustifyText::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Quit Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Styles::button(),
                                background_color: BACKGROUND_COLOR.into(),
                                ..default()
                            },
                            QuitButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style::DEFAULT,
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Quit",
                                        get_button_text_style(asset_server),
                                    )],
                                    justify: JustifyText::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    pause_menu_entity
}
