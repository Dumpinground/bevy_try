use std::f32::consts::PI;

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*, math::Vec3Swizzles,
};
use rusalka::NoiseGenerator;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TankConfig {
            tank_count: 20,
            safe_zone_radius: 8.,
        })
        .init_resource::<CannonBallMesh>()
        .add_systems(Startup, (setup, tank_spawn))
        .add_systems(Update, (tank_move, cannon_ball_velocity, check_safe_zone, turret_rotate, turret_shoot.after(turret_rotate)))
        .run();
}

#[derive(Resource)]
pub struct TankConfig {
    tank_count: u32,
    safe_zone_radius: f32,
}

#[derive(Component)]
pub struct Tank;

#[derive(Component)]
pub struct Turret {
    spawn_point: Entity,
}

#[derive(Component)]
pub struct Cannon;

#[derive(Component)]
pub struct SpawnPoint;

#[derive(Component)]
pub struct CannonBall {
    velocity: Vec3
}

#[derive(Component)]
pub struct Shooting;

#[derive(Component, Resource)]
pub struct CannonBallMesh(Handle<Mesh>);

impl FromWorld for CannonBallMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        Self(
            meshes.add(
                shape::UVSphere {
                    radius: 0.1,
                    ..default()
                }
                .into(),
            ),
        )
    }
}

fn setup(
    mut commands: Commands,
    tank_config: Res<TankConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500.).into()),
        material: materials.add(Color::GRAY.into()),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(1., 1., 1.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-50., 20., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes
                .add(
                    shape::UVSphere {
                        radius: tank_config.safe_zone_radius,
                        ..default()
                    }
                    .into(),
                )
                .into(),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.2, 0.8, 0.2, 0.4),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));
}

fn tank_spawn(
    tank_config: Res<TankConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tank_mesh = meshes.add(shape::Cube::new(1.0).into());

    let turret_mesh = meshes.add(
        shape::UVSphere {
            radius: 0.5,
            ..default()
        }
        .into(),
    );

    let cannon_mesh = meshes.add(
        shape::Cylinder {
            radius: 0.5,
            height: 2.0,
            ..default()
        }
        .into(),
    );

    for _ in 0..tank_config.tank_count {
        let material = materials.add(StandardMaterial {
            base_color: Color::hsl(rand::random::<f32>() * 360., 1., 0.5),
            ..default()
        });

        let spawn_point = commands
            .spawn((
                SpawnPoint,
                GlobalTransform::default(),
                Transform::from_xyz(0., 1., 0.),
            ))
            .id();

        let cannon = commands
            .spawn((
                Cannon,
                PbrBundle {
                    mesh: cannon_mesh.clone(),
                    transform: Transform::from_xyz(0., 0.5, 0.)
                        .with_scale(Vec3::new(0.2, 0.5, 0.2)),
                    ..default()
                },
            ))
            .add_child(spawn_point)
            .id();

        let turret = commands
            .spawn((
                Turret { spawn_point },
                PbrBundle {
                    mesh: turret_mesh.clone(),
                    material: material.clone(),
                    transform: Transform::from_xyz(0., 0.5, 0.)
                        .with_rotation(Quat::from_rotation_x(45.)),
                    ..default()
                },
            ))
            .add_child(cannon)
            .id();

        commands
            .spawn((
                Tank,
                PbrBundle {
                    mesh: tank_mesh.clone(),
                    material: material.clone(),
                    transform: Transform::from_xyz(0., 0.5, 0.),
                    ..default()
                },
            ))
            .add_child(turret);
    }
}

fn tank_move(mut tanks: Query<(Entity, &mut Transform), With<Tank>>, time: Res<Time>) {
    let dt = time.delta_seconds();
    let generator = NoiseGenerator::new("Nose");
    for (entity, mut transform) in tanks.iter_mut() {
        let mut pos = transform.translation;
        pos.y = entity.index() as f32;
        pos /= 10.;

        let angle: f32 = (0.5 + generator.get(pos.x, pos.y, pos.z)) * 4. * PI;
        let (x, z) = angle.sin_cos();
        transform.rotation = Quat::from_rotation_y(angle);
        transform.translation += Vec3::new(x, 0., z) * dt * 5.;
    }
}

fn turret_rotate(mut turret: Query<&mut Transform, With<Turret>>, time: Res<Time>) {
    let rotation_y = Quat::from_rotation_y(time.delta_seconds() * PI);

    for mut transform in turret.iter_mut() {
        transform.rotation = rotation_y * transform.rotation;
    }
}

fn turret_shoot(mut commands: Commands, cannon_ball_mesh: Res<CannonBallMesh>, turrets: Query<(&Turret, &Handle<StandardMaterial>, &GlobalTransform), With<Shooting>>, global_transform_query: Query<&GlobalTransform>) {
    for (turret, material, global_transform) in turrets.iter() {
        let spawn_point_pos = global_transform_query.get(turret.spawn_point).unwrap().translation();
        commands.spawn((CannonBall { velocity: global_transform.up() * 20. }, PbrBundle {
            material: material.clone(),
            transform: Transform::from_translation(spawn_point_pos),
            mesh: cannon_ball_mesh.0.clone(),
            ..default()
        }));
    }
}

const GRAVITY: Vec3 = Vec3::new(0., -9.82, 0.);
const INVERT_Y: Vec3 = Vec3::new(1., -1., 1.);

fn cannon_ball_velocity(mut cannon_ball: Query<(&mut CannonBall, &mut Transform, Entity)>, time: Res<Time>, mut commands: Commands) {
    let dt = time.delta_seconds();

    for (mut cannon_ball, mut transform, entity) in cannon_ball.iter_mut() {
        transform.translation += cannon_ball.velocity * dt;

        if transform.translation.y < 0. {
            transform.translation *= INVERT_Y;
            cannon_ball.velocity *= INVERT_Y * 0.8;
        }

        cannon_ball.velocity += GRAVITY * dt;

        if cannon_ball.velocity.length_squared() < 0.1 {
            commands.entity(entity).despawn();
        }
    }
}

fn check_safe_zone(turrets: Query<(Entity, &GlobalTransform, Option<&Shooting>), With<Turret>>, tank_config: Res<TankConfig>, mut commands: Commands) {
    for (entity, global_transform, shooting) in turrets.iter() {
        if global_transform.translation().xz().length() > tank_config.safe_zone_radius {
            if shooting.is_none() {
                commands.entity(entity).insert(Shooting);
            }
        } else {
            if shooting.is_some() {
                commands.entity(entity).remove::<Shooting>();
            }
        }
    }
}