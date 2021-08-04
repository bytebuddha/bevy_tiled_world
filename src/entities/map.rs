use bevy::prelude::*;

use crate::{Map, MapIndex};

#[derive(Bundle)]
pub struct MapBundle {
    pub map: Handle<Map>,
    pub visible: Visible,
    pub index: MapIndex,
}

impl Default for MapBundle {
    fn default() -> MapBundle {
        MapBundle {
            map: Default::default(),
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            index: Default::default(),
        }
    }
}
