use anyhow::{bail, Context};
use clap::{command, Parser};
use serde::Deserialize;
use std::{
    fs::{self, OpenOptions},
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};

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

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum EnemyType {
    Placeholder,
    Zombie,
    Tooth,
    Cage,
    AppleThrower,
}

#[derive(Debug)]
struct Enemy {
    type_id: EnemyType,
    spawn_point: Point,
}

#[derive(Debug)]
struct Zone {
    name: String,
    width: u16,
    height: u16,
    metatile_factor: u16,
    floor: Vec<u16>,
    ceiling: Vec<u16>,
    enemies: Vec<Enemy>,
    player_spawn_point: Point,
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
    fn from(map: &MapElement, name: String) -> anyhow::Result<Zone> {
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

fn write_header(destination: PathBuf, zone: &Zone) -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(destination)?;
    let mut writer = BufWriter::new(file);

    writeln!(&mut writer, "#pragma once")?;
    writeln!(&mut writer)?;
    writeln!(&mut writer, "namespace sp::{} {{", zone.name)?;

    writeln!(&mut writer, "\tconstexpr uint8_t width() {{ return {}; }}", zone.width)?;
    writeln!(&mut writer, "\tconstexpr uint8_t height() {{ return {}; }}", zone.height)?;

    writeln!(&mut writer, "\tconstexpr uint8_t floor_tiles[1];")?;

    writeln!(&mut writer, "}}")?;

    Ok(())
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

    let name = args
        .input
        .file_stem()
        .context("Unable to retrieve file stem")?
        .to_str()
        .context("Unable to convert file stem to string")?;
    let zone = Zone::from(&map, name.to_string())?;

    write_header(PathBuf::from(format!("sp_{}.h", name)), &zone)?;

    Ok(())
}
