mod de;
mod model;
mod se;

use anyhow::Context;
use clap::{command, Parser};
use de::*;
use model::*;
use se::*;
use std::{ffi::OsStr, fs, io::BufReader, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    input_directory: PathBuf,
    #[arg(required = true, short, long)]
    output_directory: PathBuf,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args) {
        eprintln!("Error during map conversion: {:?}", e);
        std::process::exit(1);
    }
}

fn run(args: &Args) -> anyhow::Result<()> {
    let include_dir: PathBuf = [args.output_directory.clone(), PathBuf::from("include")]
        .iter()
        .collect();
    fs::create_dir_all(include_dir)?;

    let enemy_spawn_header_path: PathBuf = [args.output_directory.clone(), PathBuf::from("include/sp_enemy_spawn.h")]
        .iter()
        .collect();
    // TODO: Would be awesome to write only when the header contents have changed
    if !enemy_spawn_header_path.exists() {
        println!("Writing {:?}", enemy_spawn_header_path);
        write_enemy_spawn_header(enemy_spawn_header_path)?;
    }

    for entry in args.input_directory.read_dir().context("Failed to read directory")? {
        let path = entry.context("Failed to get file entry")?.path();
        if path.extension() == Some(OsStr::new("tmx")) {
            convert(&path, &args.output_directory).with_context(|| format!("Failed to convert {:?}", path))?;
        }
    }

    Ok(())
}

fn convert(path: &PathBuf, output_directory: &PathBuf) -> anyhow::Result<()> {
    let file = fs::File::open(path.clone())?;
    let reader = BufReader::new(file);
    let map: MapElement = quick_xml::de::from_reader(reader)?;

    let name = path
        .file_stem()
        .context("Unable to retrieve file stem")?
        .to_str()
        .context("Unable to convert file stem to string")?;
    let zone = Zone::from(&map, name.to_string())?;

    let header_path = [
        output_directory.clone(),
        PathBuf::from(format!("include/sp_{}.h", name)),
    ]
    .iter()
    .collect();
    println!("Writing {:?}", header_path);
    write_header(header_path, &zone)?;

    Ok(())
}
