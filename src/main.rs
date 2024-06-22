use std::{fs, io::BufReader, path::PathBuf};

use anyhow::{bail, Context};
use clap::{command, Parser};
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    input: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct MapElement {
    #[serde(rename = "@width")]
    width: u16,
    #[serde(rename = "@height")]
    height: u16,
    #[serde(rename = "@tilewidth")]
    tile_width: u16,
    #[serde(rename = "@tileheight")]
    tile_height: u16,
    #[serde(default)]
    layer: Vec<LayerElement>,
    #[serde(default, rename = "objectgroup")]
    object_group: Vec<ObjectGroupElement>,
}

#[derive(Debug, Deserialize)]
struct LayerElement {
    #[serde(rename = "@id")]
    id: u16,
    #[serde(rename = "@name")]
    name: String,
    data: DataElement,
}

#[derive(Debug, Deserialize)]
struct DataElement {
    #[serde(rename = "@encoding")]
    encoding: String,
    #[serde(rename = "$text")]
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ObjectGroupElement {
    #[serde(rename = "@id")]
    id: u16,
    #[serde(rename = "@name")]
    name: String,
    #[serde(default)]
    object: Vec<ObjectElement>,
}

#[derive(Debug, Deserialize)]
struct ObjectElement {
    #[serde(rename = "@id")]
    id: u16,
    #[serde(rename = "@type")]
    type_id: String,
    #[serde(rename = "@x")]
    x: f32,
    #[serde(rename = "@y")]
    y: f32,
}

struct Point {
    x: u32,
    y: u32,
}

enum EnemyType {
    Placeholder,
    Zombie,
    Tooth,
    Cage,
    AppleThrower,
}

struct Enemy {
    type_id: EnemyType,
    spawn_point: Point,
}

struct Zone {
    width: u16,
    height: u16,
    metatile_factor: u16,
    floor: Vec<u16>,
    ceiling: Vec<u16>,
    enemies: Vec<Enemy>,
    player_spawn_point: Point,
}

impl Zone {
    fn from(map: &MapElement) -> anyhow::Result<()> {
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

        let metatile_factor = map.tile_width / 8;

        let floor_data = floor_layer
            .data
            .content
            .clone()
            .context("Missing tile data for floor layer.")?;
        let tiles_result: Result<Vec<u16>, _> = floor_data
            .replace("\n", "")
            .split(",")
            .map(|tile| {
                str::parse::<u16>(tile).context(format!("Failed to parse tile value: {}", tile))
            })
            .collect();
        let floor = tiles_result?;

        if usize::from(map.width * map.height) != floor.len() {
            bail!(
                "Incorrect floor layer size: {} vs {}",
                floor.len(),
                map.width * map.height
            );
        }

        Ok(())
    }
}

fn main() {
    let args = Args::parse();

    println!("input: {:?} output: {:?}", args.input, args.output);

    if let Err(e) = convert(&args) {
        eprintln!("Conversion failed for {:?}: {}", args.input, e);
        std::process::exit(1);
    }
}

fn convert(args: &Args) -> anyhow::Result<()> {
    let file = fs::File::open(args.input.clone())?;
    let reader = BufReader::new(file);
    let map: MapElement = quick_xml::de::from_reader(reader)?;

    println!("{:?}", map);

    let zone = Zone::from(&map)?;

    Ok(())
}
