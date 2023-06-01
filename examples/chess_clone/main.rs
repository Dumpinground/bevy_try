use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
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
            Vec3::new(-7., 20., 4.),
        )),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4., 8., 4.),
        ..default()
    });
}
