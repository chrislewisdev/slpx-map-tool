use std::{fs, io::BufReader, path::PathBuf};

use clap::{command, Parser};
use quick_xml::DeError;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    input: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>
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
    object: Vec<ObjectElement>
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

fn main() {
    let args = Args::parse();

    println!("input: {:?} output: {:?}", args.input, args.output);

    let file = fs::File::open(args.input).expect("Unable to open input file.");
    let reader = BufReader::new(file);
    let map: Result<MapElement, DeError> = quick_xml::de::from_reader(reader);

    match map {
        Err(err) => println!("{}", err),
        Ok(result) => println!("{:?}", result)
    }
}
