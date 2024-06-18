use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            EditorPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let white = materials.add(Color::WHITE);
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 10.0).looking_to(-Vec3::Z, Vec3::Y),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            transform: Transform::from_xyz(3.0, 8.0, 3.0),
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 2_000_000.0,
                ..default()
            },
            ..default()
        },
    ));
    let ground_shape = Cuboid::new(15.0, 0.25, 15.0);
    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(ground_shape)),
            material: white.clone(),
            ..default()
        },
        RigidBody::Static,
        Collider::from(ground_shape),
    ));

    let shape = Cylinder::new(0.25, 1.0);

    let mesh = meshes.add(Mesh::from(shape));
    let cylinder = |x, y| {
        (
            Name::new("Cylinder"),
            PbrBundle {
                mesh: mesh.clone(),
                material: white.clone(),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::from(shape),
        )
    };

    commands.spawn(cylinder(-0.6, 1.2));
    commands.spawn(cylinder(0.0, 1.2));
    commands.spawn(cylinder(0.6, 1.2));
    commands.spawn(cylinder(-0.3, 2.4));
    commands.spawn(cylinder(0.3, 2.4));
    commands.spawn(cylinder(0.0, 3.4));
}
