use bevy::prelude::*;
use bevy_gltf_blueprints::BlueprintsPlugin;
use bevy_registry_export::ExportRegistryPlugin;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            BlueprintsPlugin {
                legacy_mode: false,
                ..default()
            },
            ExportRegistryPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}
