use bevy::prelude::*;

use super::super::components::*;
use super::super::styles::*;

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_over_menu_entity = commands
        .spawn((Styles::game_over_menu(), ZIndex(2), GameOverMenu {}))
        .with_children(|parent| {
            parent
                .spawn((
                    Styles::game_over_menu_container(),
                    BackgroundColor(BACKGROUND_COLOR),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Game Over"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        get_title_text_font(asset_server),
                        get_text_color(),
                    ));
                    // Final Score Text
                    parent.spawn((
                        Text::new("Your final score was:"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        get_final_score_text_font(asset_server),
                        get_text_color(),
                        FinalScoreText {},
                    ));
                    // Restart Button
                    parent
                        .spawn((
                            Styles::button(),
                            Button,
                            BackgroundColor(BACKGROUND_COLOR),
                            RestartButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Restart"),
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
