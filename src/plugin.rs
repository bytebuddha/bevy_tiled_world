use bevy::prelude::*;

use crate::{
    Map, ChangeMap,
    systems::{
        spawn_tilemap, spawn_world,
        handle_map_change
    }
};

pub struct TiledWorldPlugin;

impl Plugin for TiledWorldPlugin {

    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<Map>()
            .add_asset_loader(crate::asset::TiledMapLoader)
            .add_plugin(bevy_tilemap::prelude::TilemapPlugin)
            .add_event::<ChangeMap>()
            .add_system(spawn_tilemap.system())
            .add_system(spawn_world.system())
            .add_system(handle_map_change.system());
    }
}
