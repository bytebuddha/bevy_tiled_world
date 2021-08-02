use bevy::utils::BoxedFuture;
use bevy::asset::{AssetPath, LoadedAsset, LoadContext, AssetLoader};

use super::Map;

pub struct TiledMapLoader;

impl AssetLoader for TiledMapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let map = tiled::parse_with_path(bytes, load_context.path())?;
            let mut tilesets = vec![];
            for tileset in &map.tilesets {
                for image in &tileset.images {
                    let mut path = load_context.path().parent().unwrap().to_path_buf();
                    path.push(&image.source);
                    tilesets.push(AssetPath::new(path, None));
                }
            }
            let file = load_context.path().to_path_buf();
            let loaded_asset = LoadedAsset::new(Map { map, file });
            load_context.set_default_asset(loaded_asset.with_dependencies(tilesets));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}
