use bevy::app::Events;
use bevy::prelude::*;
use bevy_tiled_world::*;

mod utils;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TiledWorldPlugin)
        .add_plugin(bevy_devtools::DevToolsPlugin::default())
        .add_startup_system(spawn_scene.system())
        .add_system(utils::move_camera.system())
        .add_system(change_map.system())
        .run()
}

fn spawn_scene(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(WorldBundle {
        maps: vec![assets.load("maps/basic.tmx"), assets.load("maps/tiny.tmx")],
        ..Default::default()
    });
}

pub fn change_map(keys: Res<Input<KeyCode>>, mut events: ResMut<Events<ChangeMap>>) {
    if keys.just_pressed(KeyCode::F1) {
        events.send(ChangeMap(0));
    }
    if keys.just_pressed(KeyCode::F2) {
        events.send(ChangeMap(1));
    }
}
