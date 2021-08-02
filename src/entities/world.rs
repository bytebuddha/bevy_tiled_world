use bevy::prelude::*;

use crate::Map;

#[derive(Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct ActiveMap(pub usize);

#[derive(Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct MapIndex(pub usize);

#[derive(Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct ChangeMap(pub usize);

#[derive(Bundle, Default)]
pub struct WorldBundle {
    pub active: ActiveMap,
    pub maps: Vec<Handle<Map>>
}
