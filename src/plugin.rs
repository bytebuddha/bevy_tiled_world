use bevy::prelude::*;

use crate::{
    systems::{handle_map_change, spawn_tilemap, spawn_world},
    ChangeMap, Map,
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
