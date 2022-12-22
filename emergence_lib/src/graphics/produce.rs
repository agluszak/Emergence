//! The [`ProduceTilemap`] manages visualization of produce.
use crate as emergence_lib;
use crate::graphics::sprites::SpriteIndex;
use crate::graphics::tilemap_marker::TilemapMarker;
use bevy::ecs::component::Component;
use bevy_ecs_tilemap::map::TilemapTileSize;
use emergence_macros::IterableEnum;

/// Enumerates produce sprites.
#[derive(Component, Clone, Copy, Hash, Eq, PartialEq, IterableEnum)]
pub enum ProduceSprite {
    /// Sprite representing food
    Food,
}

impl SpriteIndex for ProduceSprite {
    const ROOT_PATH: &'static str = "produce";

    fn leaf_path(&self) -> &'static str {
        match self {
            ProduceSprite::Food => "tile-food-balls.png",
        }
    }
}

/// Marker component for entity that manages visualization of produce.
///
/// See also:
/// * [`OrganismsTilemap`](crate::graphics::organisms::OrganismsTilemap), which lies below the
/// [`ProduceTilemap`], and manages visualization of organisms
/// * [`TerrainTilemap`](crate::graphics::terrain::TerrainTilemap), which lies on below the
/// [`OrganismsTilemap`](crate::graphics::organisms::OrganismsTilemap), and manages visualization of terrain entities
#[derive(Component, Debug, Clone, Copy)]
pub struct ProduceTilemap;

impl TilemapMarker for ProduceTilemap {
    const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 48.0, y: 54.0 };
    const MAP_Z: f32 = 2.0;
    type Index = ProduceSprite;
}