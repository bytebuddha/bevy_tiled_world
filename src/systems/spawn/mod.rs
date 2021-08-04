use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use crate::{entities::TiledWorld, ActiveMap, Map, MapBundle, MapIndex};

mod objects;
pub use self::objects::load_tiled_objects;

#[cfg(feature = "rapier")]
mod rapier;

mod utils;
pub use self::utils::{build_texture_atlas, get_grid_topology, get_tileset, merge_paths};

pub const Z_SEPERATION: f32 = 100.0;

pub fn spawn_world(
    mut commands: Commands,
    query: Query<(Entity, &Vec<Handle<Map>>, &ActiveMap), Without<TiledWorld>>,
) {
    for (entity, world, active) in query.iter() {
        commands
            .entity(entity)
            .remove::<Visible>()
            .insert(Name::new("Tiled World"))
            .insert(TiledWorld)
            .with_children(|parent| {
                for (dex, map) in world.iter().enumerate() {
                    let visible = if dex == active.0 {
                        Visible {
                            is_visible: true,
                            is_transparent: true,
                        }
                    } else {
                        Visible {
                            is_visible: false,
                            is_transparent: true,
                        }
                    };
                    parent.spawn_bundle(MapBundle {
                        map: map.clone(),
                        index: MapIndex(dex),
                        visible,
                    });
                }
            });
    }
}

pub fn spawn_tilemap(
    maps: Res<Assets<Map>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<(Entity, &Handle<Map>, &Visible, &MapIndex), Without<TiledWorld>>,
) {
    for (entity, map, visible, index) in query.iter() {
        if let Some(map) = maps.get(map) {
            commands
                .entity(entity)
                .insert(TiledWorld)
                .insert(Name::new(format!("Map({})", map.file.to_str().unwrap())))
                .insert(Transform::identity())
                .insert(GlobalTransform::identity())
                .with_children(|parent| {
                    for (dex, layer) in map.map.layers.iter().enumerate() {
                        let atlas = build_texture_atlas(&*assets, &map, &layer);
                        let atlas_handle = atlases.add(atlas);
                        let mut tilemap = Tilemap::builder()
                            .dimensions(map.map.width, map.map.height)
                            .topology(get_grid_topology(&map.map))
                            .chunk_dimensions(map.map.tile_width, map.map.tile_height, 1)
                            .texture_dimensions(map.map.tile_width, map.map.tile_height)
                            .texture_atlas(atlas_handle.clone())
                            .auto_chunk()
                            .auto_spawn(2, 2)
                            .finish()
                            .expect(&format!(
                                "Failed to build Tilemap for Map({})",
                                map.file.to_str().unwrap()
                            ));
                        tilemap
                            .insert_tiles(import_tiles(&map.map, &layer.tiles))
                            .unwrap();
                        parent
                            .spawn_bundle(TilemapBundle {
                                tilemap,
                                visible: visible.clone(),
                                transform: Transform::from_translation(Vec3::new(
                                    0.0,
                                    0.0,
                                    dex as f32 * Z_SEPERATION,
                                )),
                                global_transform: GlobalTransform::from_translation(Vec3::new(
                                    0.0,
                                    0.0,
                                    dex as f32 * Z_SEPERATION,
                                )),
                            })
                            .insert(layer.clone())
                            .insert(*index)
                            .insert(Name::new(format!("Layer({})", layer.name)));
                    }
                    load_tiled_objects(parent, &map.map);
                });
        }
    }
}

fn import_tiles(map: &tiled::Map, tiles: &tiled::LayerData) -> Vec<Tile<(i32, i32)>> {
    let mut out_tiles = vec![];
    match tiles {
        tiled::LayerData::Finite(value) => {
            for (chunk_x, chunks) in value.iter().enumerate() {
                for (chunk_y, tile) in chunks.iter().enumerate() {
                    if tile.gid > 0 {
                        if let Some(tileset) = map.get_tileset_by_gid(tile.gid) {
                            out_tiles.push(Tile {
                                point: (chunk_y as i32, map.height as i32 - chunk_x as i32).into(),
                                sprite_index: tile.gid as usize - tileset.first_gid as usize,
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
        tiled::LayerData::Infinite(data) => {
            for ((y, x), chunk) in data.iter() {
                for (chunk_x, row) in chunk.tiles.iter().enumerate() {
                    for (chunk_y, tile) in row.iter().enumerate() {
                        if let Some(tileset) = map.get_tileset_by_gid(tile.gid) {
                            out_tiles.push(Tile {
                                point: (*y + chunk_y as i32, -(chunk_x as i32 + *x)).into(),
                                sprite_index: tile.gid as usize - tileset.first_gid as usize,
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
    }
    out_tiles
}
