use bevy::{
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems((setup.on_startup(),))
        .run()
}

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("translation", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        },
        AnimateTranslation,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("rotation", text_style.clone()).with_alignment(text_alignment),
            ..default()
        },
        AnimateRotation,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("scale", text_style).with_alignment(text_alignment),
            ..default()
        },
        AnimateScale,
    ));

    let slightly_smaller_text_style = TextStyle {
        font,
        font_size: 42.0,
        color: Color::WHITE,
    };

    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n",
                        slightly_smaller_text_style.clone(),
                    )],
                    alignment: TextAlignment::Left,
                    linebreak_behaviour: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds { size: box_size },
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    let other_box_size = Vec2::new(300.0, 200.0);
    let other_box_position = Vec2::new(320.0, -250.0);
}

fn animate_translation() {}

fn animate_rotation() {}

fn animate_scale() {}
