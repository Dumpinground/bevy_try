use bevy::prelude::*;

use super::super::{components::*, styles::*};

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((Styles::hud(), HUD {}))
        .with_children(|parent| {
            // LHS
            parent
                .spawn((Styles::lhs(), BackgroundColor(BACKGROUND_COLOR)))
                .with_children(|parent| {
                    // Star Image
                    parent.spawn((
                        Styles::image(),
                        ImageNode {
                            image: asset_server.load("ball-game/sprites/star.png"),
                            ..default()
                        },
                    ));
                    // Score Text
                    parent.spawn((
                        Text::new("0"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        get_text_font(asset_server),
                        get_text_color(),
                        ScoreText {},
                    ));
                });
            // RHS
            parent
                .spawn((Styles::rhs(), BackgroundColor(BACKGROUND_COLOR)))
                .with_children(|parent| {
                    // Enemy Text
                    parent.spawn((
                        Text::new("0"),
                        TextLayout::new_with_justify(JustifyText::Center),
                        get_text_font(asset_server),
                        get_text_color(),
                        ScoreText {},
                        EnemyText {},
                    ));
                    // Enemy Image
                    parent.spawn((
                        Styles::image(),
                        ImageNode {
                            image: asset_server.load("ball-game/sprites/ball_red_large.png"),
                            ..default()
                        },
                    ));
                });
        })
        .id();

    hud_entity
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
