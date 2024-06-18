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
        .insert_gizmo_group(
            PhysicsGizmos::default(),
            GizmoConfig {
                depth_bias: -1.0,
                ..default()
            },
        )
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

    let shape = Cuboid::from_size(Vec3::splat(1.0));
    let shape_half = Cuboid::from_size(Vec3::splat(0.5));

    let mesh = meshes.add(Mesh::from(shape));
    let mesh_half = meshes.add(Mesh::from(shape_half));
    let pbr = |transform| PbrBundle {
        mesh: mesh.clone(),
        material: white.clone(),
        transform,
        ..default()
    };
    let pbr_half = |transform| PbrBundle {
        mesh: mesh_half.clone(),
        material: white.clone(),
        transform,
        ..default()
    };
    let transform_index = |i: usize| Transform::from_xyz(i as f32 * 2.0 - 4.0, 0.7, 0.0);

    commands.spawn((
        Name::new("Object A"),
        pbr(transform_index(0)),
        RigidBody::Static,
        Collider::from(shape),
    ));

    commands
        .spawn((
            Name::new("Object B"),
            RigidBody::Static,
            SpatialBundle {
                transform: transform_index(1),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((pbr(Transform::IDENTITY), Collider::from(shape)));
        });

    commands
        .spawn((
            Name::new("Object C"),
            RigidBody::Static,
            SpatialBundle {
                transform: transform_index(2).with_scale(Vec3::splat(2.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((pbr_half(Transform::IDENTITY), Collider::from(shape_half)));
        });

    // Potential bug: this object does not inherit its parent's scale
    commands
        .spawn((
            Name::new("Object D"),
            SpatialBundle {
                transform: transform_index(3).with_scale(Vec3::splat(2.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                RigidBody::Static,
                pbr_half(Transform::IDENTITY),
                Collider::from(shape_half),
            ));
        });
}
