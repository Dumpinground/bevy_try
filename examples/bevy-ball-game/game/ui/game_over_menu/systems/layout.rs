use bevy::prelude::*;

use super::super::components::*;
use super::super::styles::*;

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_over_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Styles::game_over_menu(),
                z_index: ZIndex::Local(2),
                ..default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Styles::game_over_menu_container(),
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Game Over",
                                get_title_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Final Score Text
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Your final score was:",
                                    get_final_score_text_style(asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        FinalScoreText {},
                    ));
                    // Restart Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Styles::button(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            RestartButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style::DEFAULT,
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Restart",
                                        get_button_text_style(asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
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
                                background_color: NORMAL_BUTTON.into(),
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
                                    alignment: TextAlignment::Center,
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
                                background_color: NORMAL_BUTTON.into(),
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
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    game_over_menu_entity
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}
