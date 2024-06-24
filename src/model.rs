use crate::de::*;
use anyhow::{bail, Context};

#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub enum EnemyType {
    Placeholder,
    Zombie,
    Tooth,
    Cage,
    AppleThrower,
}

#[derive(Debug)]
pub struct Enemy {
    pub type_id: EnemyType,
    pub spawn_point: Point,
}

#[derive(Debug)]
pub struct Zone {
    pub name: String,
    pub width: u16,
    pub height: u16,
    pub metatile_factor: u16,
    pub floor: Vec<u16>,
    pub ceiling: Vec<u16>,
    pub enemies: Vec<Enemy>,
    pub player_spawn_point: Point,
}

fn layer_to_tiles(map: &MapElement, layer: &LayerElement) -> anyhow::Result<Vec<u16>> {
    let data = layer
        .data
        .content
        .clone()
        .context("Missing tile data for floor layer.")?;
    let metatiles_result: Result<Vec<u16>, _> = data
        .replace("\n", "")
        .split(",")
        .map(|tile| {
            str::parse::<u16>(tile).context(format!("Failed to parse tile value: {}", tile))
        })
        .collect();
    let metatiles = metatiles_result?;

    if usize::from(map.width * map.height) != metatiles.len() {
        bail!(
            "Incorrect floor layer size: {} vs {}",
            metatiles.len(),
            map.width * map.height
        );
    }

    // Expand the metatiles into 8x8 tiles
    let metatile_factor = map.tile_width / 8;
    let mt_squared = metatile_factor * metatile_factor;
    let tiles: Vec<u16> = metatiles
        .iter()
        .flat_map(|&tile| (tile * mt_squared..tile * mt_squared + mt_squared).collect::<Vec<u16>>())
        .collect();

    Ok(tiles)
}

impl Zone {
    pub fn from(map: &MapElement, name: String) -> anyhow::Result<Zone> {
        let floor_layer = map
            .layer
            .iter()
            .find(|l| l.name == "Floor")
            .context("Missing tile layer 'Floor'")?;
        let ceiling_layer = map
            .layer
            .iter()
            .find(|l| l.name == "Ceiling")
            .context("Missing tile layer 'Ceiling'")?;

        let floor = layer_to_tiles(map, &floor_layer)?;
        let ceiling = layer_to_tiles(map, &ceiling_layer)?;

        let metatile_factor = map.tile_width / 8;
        Ok(Zone {
            name,
            width: map.width,
            height: map.height,
            metatile_factor,
            floor,
            ceiling,
            enemies: vec![],
            player_spawn_point: Point { x: 0, y: 0 },
        })
    }
}
