use bevy::reflect::TypeUuid;

use std::path::PathBuf;

#[derive(Debug, TypeUuid)]
#[uuid = "c8c5cb41-20fb-4229-a8cd-3015a27c3f99"]
pub struct Map {
    pub file: PathBuf,
    pub map: tiled::Map,
}
