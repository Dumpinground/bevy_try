// reference: https://mbuffett.com/posts/bevy-snake-tutorial/#0.3

use bevy::prelude::*;
use rand::prelude::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".to_string(),
                resolution: (500., 500.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .insert_resource(Time::<Fixed>::from_seconds(1.))
        .add_systems(Startup, (setup_camera, spawn_snake))
        .add_systems(Update, snake_movement)
        .add_systems(FixedUpdate, spawn_food)
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::srgb(1., 0., 1.);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Component)]
struct SnakedHead {
    direction: Direction,
}

#[derive(Component)]
struct Food;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

fn spawn_snake(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: SNAKE_HEAD_COLOR,
            ..default()
        },
        SnakedHead {
            direction: Direction::Up,
        },
        Position { x: 3, y: 3 },
        Size::square(0.8),
    ));
}

fn spawn_food(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: FOOD_COLOR,
            ..default()
        },
        Food,
        Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        },
        Size::square(0.8),
    ));
}

fn snake_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Position, With<SnakedHead>>,
) {
    for mut pos in head_positions.iter_mut() {
        if input.pressed(KeyCode::ArrowLeft) {
            pos.x -= 1;
        }
        if input.pressed(KeyCode::ArrowRight) {
            pos.x += 1;
        }
        if input.pressed(KeyCode::ArrowDown) {
            pos.y -= 1;
        }
        if input.pressed(KeyCode::ArrowUp) {
            pos.y += 1;
        }
    }
}

fn size_scaling(window: Single<&Window>, mut q: Query<(&Size, &mut Transform)>) {
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width(),
            sprite_size.height / ARENA_HEIGHT as f32 * window.height(),
            1.,
        );
    }
}

fn position_translation(window: Single<&Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let title_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (title_size / 2.)
    }

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
            0.,
        );
    }
}
