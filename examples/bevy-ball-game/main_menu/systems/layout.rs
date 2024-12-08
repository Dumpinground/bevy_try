use bevy::prelude::*;

use super::super::components::*;
use super::super::styles::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_manu_entity = commands
        .spawn((Styles::main_menu(), MainMenu {}))
        .with_children(|parent| {
            // === Title ===
            parent.spawn(Styles::title()).with_children(|parent| {
                // Image 1
                parent.spawn((
                    Styles::image(),
                    ImageNode {
                        image: asset_server.load("ball-game/sprites/ball_blue_large.png"),
                        ..default()
                    },
                ));
                // Text
                parent.spawn((
                    Text::new("Bevy Ball Game"),
                    TextLayout::new_with_justify(JustifyText::Center),
                    get_title_text_font(asset_server),
                    get_text_color(),
                ));

                // Image 2
                parent.spawn((
                    Styles::image(),
                    ImageNode {
                        image: asset_server.load("ball-game/sprites/ball_red_large.png"),
                        ..default()
                    },
                ));
            });

            // ===Play Button===
            parent
                .spawn((
                    Styles::button(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    Button,
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        get_button_text_font(asset_server),
                        get_text_color(),
                    ));
                });

            // ===Quit Button===
            parent
                .spawn((
                    Styles::button(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    Button,
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
        })
        .id();

    main_manu_entity
}
