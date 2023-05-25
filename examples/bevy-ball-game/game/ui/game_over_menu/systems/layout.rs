use bevy::prelude::*;

use super::super::components::*;
use super::super::styles::*;

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_over_menu_entity = commands
        .spawn((NodeBundle {
            style: GAME_OVER_MENU_STYLE,
            z_index: ZIndex::Local(2),
            ..default(),
        }, 
            GameOverMenu {}, 
        )).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: GAME_OVER_MENU_CONTAINER_STYLE,
                background_color: BACKGROUND_COLOR,
                ..default()
            })
            .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text { sections: vec![TextSection::new("Game Over", get_title_text_style(&asset_server))], alignment: TextAlignment::Center, ..default() }, ..default()
                    });
                    // Final Score Text
                    parent.spawn((TextBundle {
                        text: Text { sections: vec![TextSection::new("Your final score was:", get_final_score_text_style(&asset_server))], alignment: TextAlignment::Center,..default() }, ..default()
                    }, FinalScoreText {} ));
                    // Restart Button
                    parent.spawn((ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    }, RestartButton {}));
                    // Main Menu Button
                    parent.spawn((ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    }, MainMenuButton {}));
                    // Quit Button
                    parent.spawn((ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    }, QuitButton {}));
                });
        }).id();

    game_over_menu_entity
}