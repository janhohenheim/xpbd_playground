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

    let body_shape = Cuboid::new(1.0, 1.0, 0.5);
    let lid_shape = Cuboid::new(1.0, 1.0, 0.1);

    let body_mesh = meshes.add(Mesh::from(body_shape));
    let lid_mesh = meshes.add(Mesh::from(lid_shape));

    let body = commands
        .spawn((
            Name::new("Body"),
            PbrBundle {
                mesh: body_mesh,
                material: white.clone(),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::from(body_shape),
        ))
        .id();

    let lid = commands
        .spawn((
            Name::new("Lid"),
            PbrBundle {
                mesh: lid_mesh,
                material: white.clone(),
                transform: Transform::from_xyz(0.0, 1.5, 0.0).looking_at(Vec3::X, Vec3::Y),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::from(lid_shape),
        ))
        .id();

    commands.spawn((
        Name::new("Joint"),
        RevoluteJoint::new(body, lid)
            .with_local_anchor_1(Vec3::new(0.0, 0.5, 0.0))
            .with_local_anchor_2(Vec3::new(0.0, -0.5, 0.0)),
    ));
}
