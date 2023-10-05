use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_floor));
    }
}

fn spawn_light(mut commands: Commands) {
    let light = (PointLightBundle {
        point_light: PointLight {
            // 246, 218, 111, 255
            color: Color::rgba(1., 0.78, 0., 1.),
            intensity: 100.,
            ..default()
        },
        transform: Transform::from_xyz(0., 5., 0.),
        ..default()
    }, Name::new("Main Light"));

    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    }, Name::new("Floor"));

    commands.spawn(floor);
}
