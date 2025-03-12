#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, (spawn_cube_timer, despawn_cubes, display_cube_count))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.5, 5.5, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn setup_physics(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<StandardMaterial>>,) {
    let r = rand::random::<u8>();
    let g = rand::random::<u8>();
    let b = rand::random::<u8>();
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(10.0, 0.1))),
        RigidBody::Fixed,
        Restitution::coefficient(0.4),
        Collider::cylinder(0.05, 10.0),
        MeshMaterial3d(materials.add(Color::srgb_u8(r, g, b))),
        Transform::from_xyz(0.0, -2.0, 0.0))
    );
}

fn spawn_cube_timer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut next_spawn: Local<f32>,
) {
    if time.elapsed_secs() > *next_spawn {
        // 每200ms生成一次
        *next_spawn = time.elapsed_secs() + 0.1;
        
        // 生成随机颜色
        let r = rand::random::<u8>();
        let g = rand::random::<u8>();
        let b = rand::random::<u8>();
        
        // 生成随机角度和半径
        let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let radius = rand::random::<f32>() * 4.0;
        
        // 计算x和z坐标
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.6, 0.6, 0.6))),
            RigidBody::Dynamic,
            Restitution::coefficient(0.6),
            Collider::cuboid(0.3, 0.3, 0.3),
            MeshMaterial3d(materials.add(Color::srgb_u8(r, g, b))),
            //Velocity::angular(Vec3::new(1.0, 1.0, 1.0)),
            Transform::from_xyz(x, 20.0, z),
        ));
    }
}

fn despawn_cubes(
    mut commands: Commands,
    query: Query<(Entity, &Transform)>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -20.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn display_cube_count(
    query: Query<&Transform>,
    mut text: Single<&mut Text>,
) {
    let count = query.iter().filter(|transform| transform.translation.y > -20.0).count();
    text.0 = "Cube Count: ".to_owned() + &count.to_string();
}
