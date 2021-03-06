use bevy::prelude::*;
use bevy_tiled_world::*;

mod utils;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TiledWorldPlugin)
        .add_plugin(bevy_devtools::DevToolsPlugin)
        .add_startup_system(spawn_scene.system())
        .add_system(utils::move_camera.system())
        .run()
}

fn spawn_scene(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(MapBundle {
        map: assets.load("maps/basic.tmx"),
        ..Default::default()
    });
}
