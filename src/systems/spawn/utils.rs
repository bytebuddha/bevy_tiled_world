use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use std::path::PathBuf;

use crate::Map;

pub fn get_grid_topology(map: &tiled::Map) -> GridTopology {
    match map.orientation {
        tiled::Orientation::Orthogonal => GridTopology::Square,
        tiled::Orientation::Hexagonal => GridTopology::HexEvenCols,
        tiled::Orientation::Isometric => GridTopology::Square,
        tiled::Orientation::Staggered => GridTopology::Square
    }
}

pub fn get_tileset<'a>(map: &'a Map, layer: &tiled::Layer) -> &'a tiled::Tileset {
    match &layer.tiles {
        tiled::LayerData::Finite(data) => {
            for row in data.iter() {
                for column in row.iter() {
                    if column.gid > 0 {
                        return map.map.get_tileset_by_gid(column.gid).unwrap();
                    }
                }
            }
        },
        tiled::LayerData::Infinite(data) => {
            for (_, row) in data.iter() {
                for chunk in row.tiles.iter() {
                    for tile in chunk.iter() {
                        if tile.gid > 0 {
                            return map.map.get_tileset_by_gid(tile.gid).unwrap();
                        }
                    }
                }
            }
        }
    }
    &map.map.tilesets[0]
}

pub fn build_texture_atlas(assets: &AssetServer, map: &Map, layer: &tiled::Layer) -> TextureAtlas {
    let tileset = get_tileset(map, layer);
    let first_tileset_image = &tileset.images[0];
    TextureAtlas::from_grid(
        assets.load(merge_paths(map.file.clone(), (&tileset.images[0].source).into())),
        Vec2::new(tileset.tile_width as f32, tileset.tile_height as f32),
        first_tileset_image.width as usize / tileset.tile_width as usize,
        first_tileset_image.height as usize / tileset.tile_height as usize
    )
}

pub fn merge_paths(mut image: PathBuf, mut relative_to: PathBuf) -> PathBuf {
    image.pop();
    loop {
        if relative_to.starts_with("..") {
            image.pop();
            image = image;
            relative_to = relative_to.into_iter().skip(1).collect();
        } else {
            image.push(&relative_to);
            break
        }
    }
    image
}
