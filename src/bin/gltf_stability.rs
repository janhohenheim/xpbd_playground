use bevy::{gltf::Gltf, prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_gltf_blueprints::{BlueprintsPlugin, GltfBlueprintsSet};
use bevy_registry_export::ExportRegistryPlugin;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            BlueprintsPlugin {
                legacy_mode: false,
                library_folder: "library".into(),
                ..default()
            },
            ExportRegistryPlugin::default(),
            EditorPlugin::default(),
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Running)
                .load_collection::<GltfAssets>(),
        )
        .add_systems(OnExit(GameState::Loading), setup)
        .add_systems(Update, load_collider.after(GltfBlueprintsSet::AfterSpawn))
        .register_type::<SerializableCollider>()
        .run();
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Running,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct GltfAssets {
    #[asset(path = "world.glb")]
    pub(crate) level: Handle<Gltf>,
    #[asset(path = "library", collection(typed, mapped))]
    pub(crate) _library: HashMap<String, Handle<Gltf>>,
}

fn setup(mut commands: Commands, gltfs: Res<Assets<Gltf>>, gltf_assets: Res<GltfAssets>) {
    let gltf = gltfs.get(&gltf_assets.level).unwrap();

    commands.spawn((
        Name::new("Level"),
        SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..default()
        },
    ));
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
}

fn load_collider(mut commands: Commands, q_colliders: Query<(Entity, &SerializableCollider)>) {
    for (entity, collider) in q_colliders.iter() {
        match *collider {
            SerializableCollider::Cylinder { height, radius } => {
                let shape = Cylinder::new(radius, height);
                commands.entity(entity).insert(Collider::from(shape));
            }
            SerializableCollider::Cuboid {
                x_length,
                y_length,
                z_length,
            } => {
                let shape = Cuboid::new(x_length, y_length, z_length);
                commands.entity(entity).insert(Collider::from(shape));
            }
        }
        commands.entity(entity).remove::<SerializableCollider>();
    }
}

#[derive(Debug, Component, Copy, Clone, Reflect)]
#[reflect(Component)]
enum SerializableCollider {
    Cylinder {
        height: f32,
        radius: f32,
    },
    Cuboid {
        x_length: f32,
        y_length: f32,
        z_length: f32,
    },
}
