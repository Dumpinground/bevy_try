use bevy::{input::mouse::MouseMotion, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // .add_system(camera_controller)
        .add_systems(Update, (key_event, light_moving))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(7., 7.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // Cubes
    let cube_mesh = Mesh3d(meshes.add(Cuboid::default()));
    let cube_material = MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6)));

    let translations = [
        Vec3::new(0.0, 0.5, 0.0),
        Vec3::new(2.0, 0.5, 0.0),
        Vec3::new(-2.0, 0.5, 0.0),
        Vec3::new(0.0, 0.5, 2.0),
        Vec3::new(0.0, 0.5, -2.0),
    ];

    for t in translations {
        commands.spawn((
            cube_mesh.clone(),
            cube_material.clone(),
            Transform::from_translation(t),
        ));
    }

    // light
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0),
        Light::default(),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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

            key_up: KeyCode::ArrowUp,
            key_down: KeyCode::ArrowDown,

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

        transform.translation += direction * time.delta_secs();
    }
}

fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse_events.read() {
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut CameraController)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, camera)) = camera_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyQ) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyE) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * camera.walk_speed * time.delta_secs();
    }
}
