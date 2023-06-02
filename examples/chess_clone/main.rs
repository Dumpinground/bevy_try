use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".into(),
                resolution: (800., 400.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_startup_system(create_board)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(7.0).into()),
        material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
        transform: Transform::from_xyz(4., 0., 4.),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7., 13., 4.),
        )),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4., 8., 4.),
        ..default()
    });
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(shape::Plane::from_size(1.).into());
    let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

    for i in 0..8 {
        for j in 0..8 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: if (i + j + 1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_xyz(i as f32, 0., j as f32),
                ..default()
            });
        }
    }
}
