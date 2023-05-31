use bevy::{input::mouse::MouseMotion, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_system(camera_controller)
        .add_system(key_event)
        .add_system(light_moving)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(7.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(2.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-2.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 2.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, -2.0),
        ..default()
    });

    // light
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
        Light::default(),
    ));
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
    ));
}

#[derive(Component)]
struct Light {
    pub speed: f32,
}

impl Default for Light {
    fn default() -> Self {
        Self { speed: 2.0 }
    }
}

#[derive(Component)]
struct CameraController {
    pub enabled: bool,

    pub key_up: KeyCode,
    pub key_down: KeyCode,

    pub walk_speed: f32,
    pub run_speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,

            key_up: KeyCode::Up,
            key_down: KeyCode::Down,

            walk_speed: 3.0,
            run_speed: 5.0,
        }
    }
}

fn light_moving(time: Res<Time>, mut light_query: Query<(&mut Transform, &mut Light)>) {
    if let Ok((mut transform, mut light)) = light_query.get_single_mut() {
        let position = transform.translation;
        let mut direction = Vec3::ZERO;
        if position.x < -10.0 || position.x > 10.0 {
            light.speed = -light.speed;
        }

        direction += Vec3::new(1.0, 0.0, 0.0) * light.speed;

        transform.translation += direction * time.delta_seconds();
    }
}

fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse_events.iter() {
        mouse_delta += mouse_event.delta;
    }

    for (mut transform, mut options) in &mut query {
        if !options.enabled {
            continue;
        }

        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }

        if axis_input != Vec3::ZERO {
            let max_speed = options.walk_speed;
        };

        if mouse_delta != Vec2::ZERO {}
    }
}

fn key_event(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut CameraController)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, camera)) = camera_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Q) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::E) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * camera.walk_speed * time.delta_seconds();
    }
}

