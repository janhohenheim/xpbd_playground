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
        .add_systems(Update, collision_detection)
        .run();
}

#[derive(Debug, Component)]
struct Ground;

fn setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
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

    let sensor_shape = Sphere::new(1.0);
    let obj_shape = Cuboid::from_size(Vec3::splat(0.5));
    let mesh = meshes.add(Mesh::from(obj_shape));
    let mut object = || {
        (
            PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(Color::WHITE),
                ..default()
            },
            Collider::from(obj_shape),
        )
    };
    let transform_index = |i: usize| Transform::from_xyz(i as f32 * 2.5 - 2.0, 0.7, 0.0);

    commands
        .spawn((
            Name::new("Object A"),
            SpatialBundle::from_transform(transform_index(0)),
        ))
        .with_children(|parent| {
            parent.spawn(Collider::from(sensor_shape));
            parent.spawn((object(), RigidBody::Static));
        });

    // Potential bug: this object never reports a collision
    commands
        .spawn((
            Name::new("Object B"),
            SpatialBundle::from_transform(transform_index(1)),
        ))
        .with_children(|parent| {
            parent.spawn(Collider::trimesh_from_mesh(&Mesh::from(sensor_shape)).unwrap());
            parent.spawn((object(), RigidBody::Static));
        });

    commands
        .spawn((
            Name::new("Object C"),
            SpatialBundle::from_transform(transform_index(2)),
        ))
        .with_children(|parent| {
            parent.spawn(Collider::from(sensor_shape));
            parent
                .spawn((object(), RigidBody::Static))
                .insert(Transform::from_xyz(1.0, 0.0, 0.0));
        });
}

fn collision_detection(
    mut collisions: EventReader<Collision>,
    q_materials: Query<&Handle<StandardMaterial>, Without<Ground>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for Collision(contacts) in collisions.read() {
        let Ok(material_handle) = q_materials
            .get(contacts.entity1)
            .or_else(|_| q_materials.get(contacts.entity2))
        else {
            continue;
        };

        let material = materials.get_mut(material_handle).unwrap();
        if contacts.during_current_frame {
            material.base_color = Color::GREEN;
        } else {
            material.base_color = Color::RED;
        }
    }
}
