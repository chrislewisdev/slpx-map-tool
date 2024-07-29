use std::str::FromStr;

use crate::de::*;
use anyhow::{bail, Context};

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub enum EnemyType {
    Tooth,
    Cage,
    Creeper,
    Thrower,
}

#[derive(Debug)]
pub struct Enemy {
    pub type_id: EnemyType,
    pub spawn_point: Point,
}

#[derive(Debug)]
pub struct Portal {
    pub target_zone: String,
    pub position: Point,
    pub width: i32,
    pub height: i32,
    pub destination: Point,
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
    pub portals: Vec<Portal>,
}

fn layer_to_tiles(map: &MapElement, layer: &LayerElement) -> anyhow::Result<Vec<u16>> {
    let data = layer
        .data
        .content
        .as_ref()
        .context("Missing tile data for floor layer.")?;
    let metatiles_result: Result<Vec<u16>, _> = data
        .replace("\n", "")
        .replace("\r", "")
        .split(",")
        .map(|tile| str::parse::<u16>(tile).context(format!("Failed to parse tile value: {}", tile)))
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
        // Tiled uses 0 for 'null' tiles, so all real tile values are offset by 1
        .map(|&tile| if tile > 0 { tile - 1 } else { tile })
        .flat_map(|tile| (tile * mt_squared..tile * mt_squared + mt_squared).collect::<Vec<u16>>())
        .collect();

    Ok(tiles)
}

impl FromStr for EnemyType {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<EnemyType> {
        match input {
            "tooth" => Ok(EnemyType::Tooth),
            "cage" => Ok(EnemyType::Cage),
            "creeper" => Ok(EnemyType::Creeper),
            "thrower" => Ok(EnemyType::Thrower),
            _ => bail!("Unrecognised enemy type"),
        }
    }
}

impl EnemyType {
    pub fn to_str(self: &EnemyType) -> &str {
        match self {
            EnemyType::Tooth => "tooth",
            EnemyType::Cage => "cage",
            EnemyType::Creeper => "creeper",
            EnemyType::Thrower => "thrower",
        }
    }
}

impl Enemy {
    pub fn from(object: &ObjectElement, map_half_width: i32, map_half_height: i32) -> anyhow::Result<Enemy> {
        Ok(Enemy {
            type_id: EnemyType::from_str(object.type_id.as_str())?,
            spawn_point: Point {
                x: object.x.floor() as i32 - map_half_width,
                y: map_half_height - object.y.floor() as i32,
            },
        })
    }
}

impl Portal {
    pub fn from(object: &ObjectElement, map_half_width: i32, map_half_height: i32) -> anyhow::Result<Portal> {
        let props = &object.properties.as_ref().context("Missing properties for portal")?.properties;
        let zone = props
            .iter()
            .find(|p| p.name == "zone")
            .context("Missing property 'zone' on portal")?;
        let x_prop = props
            .iter()
            .find(|p| p.name == "x")
            .context("Missing property 'x' on portal")?;
        let y_prop = props
            .iter()
            .find(|p| p.name == "y")
            .context("Missing property 'y' on portal")?;
        let x = str::parse::<i32>(&x_prop.value)
            .context(format!("Failed to parse '{}' as x value for portal", x_prop.value))?;
        let y = str::parse::<i32>(&y_prop.value)
            .context(format!("Failed to parse '{}' as y value for portal", y_prop.value))?;

        Ok(Portal {
            target_zone: zone.value.clone(),
            position: Point {
                x: object.x.floor() as i32 - map_half_width,
                y: map_half_height - object.y.floor() as i32,
            },
            width: 50,
            height: 50,
            destination: Point {
                x: x - map_half_width,
                y: map_half_height - y,
            },
        })
    }
}

impl Zone {
    pub fn from(map: &MapElement, name: String) -> anyhow::Result<Zone> {
        let half_width: i32 = (map.width * map.tile_width / 2) as i32;
        let half_height = (map.height * map.tile_width / 2) as i32;

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

        let spawn_layer = map
            .object_groups
            .iter()
            .find(|g| g.name == "Spawn")
            .context("Missing object group 'Spawn'")?;
        let spawn_object = spawn_layer.object.get(0).context("Missing object for spawn point")?;
        let spawn_point = Point {
            x: spawn_object.x.floor() as i32 - half_width,
            y: half_height - spawn_object.y.floor() as i32,
        };

        let enemies_layer = map
            .object_groups
            .iter()
            .find(|g| g.name == "Enemies")
            .context("Missing object group 'Enemies'")?;
        let enemies: Result<Vec<Enemy>, _> = enemies_layer
            .object
            .iter()
            .map(|o| Enemy::from(o, half_width, half_height))
            .collect();

        let portals_layer = map
            .object_groups
            .iter()
            .find(|g| g.name == "Portals");
        let portals: Result<Vec<Portal>, _> = match portals_layer {
            Some(layer) => layer
            .object
            .iter()
            .map(|o| Portal::from(o, half_width, half_height))
            .collect(),
            None => Ok(Vec::new())
        };

        let metatile_factor = map.tile_width / 8;
        Ok(Zone {
            name,
            width: map.width,
            height: map.height,
            metatile_factor,
            floor,
            ceiling,
            enemies: enemies?,
            player_spawn_point: spawn_point,
            portals: portals?,
        })
    }
}
