use bevy::prelude::*;
use bevy_tiled_world::*;

mod utils;

#[cfg(feature = "rapier")]
fn main() {
    use bevy_rapier2d::prelude::*;
    App::build()
        .init_resource::<RapierConfiguration>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TiledWorldPlugin)
        .add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(bevy_devtools::DevToolsPlugin::default())
        .add_startup_system(spawn_scene.system())
        .add_system(utils::move_camera.system())
        .add_system(utils::colliders::draw_colliders.system())
        .run()
}

#[cfg(not(feature = "rapier"))]
fn main() {
    panic!("This example requires the rapier feature be enabled")
}

fn spawn_scene(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(MapBundle {
        map: assets.load("maps/rapier.tmx"),
        ..Default::default()
    });
}
