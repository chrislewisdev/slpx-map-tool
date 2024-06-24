use crate::model::*;
use std::{
    fs::OpenOptions, io::{BufWriter, Write}, path::PathBuf
};

pub fn write_header(destination: PathBuf, zone: &Zone) -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(destination)?;
    let mut writer = BufWriter::new(file);

    writeln!(&mut writer, "#pragma once")?;
    writeln!(&mut writer)?;
    writeln!(&mut writer, "#include \"bn_core.h\"")?;
    writeln!(&mut writer)?;
    writeln!(&mut writer, "namespace sp::{} {{", zone.name)?;

    writeln!(&mut writer, "\tconstexpr uint8_t width() {{ return {}; }}", zone.width * zone.metatile_factor)?;
    writeln!(&mut writer, "\tconstexpr uint8_t height() {{ return {}; }}", zone.height * zone.metatile_factor)?;

    let floor_tiles_csv: Vec<String> = zone.floor.iter().map(|t| t.to_string()).collect();
    writeln!(&mut writer, "\tconstexpr uint8_t floor_tiles[{}] = {{", zone.width * zone.height * zone.metatile_factor * zone.metatile_factor)?;
    writeln!(&mut writer, "\t\t{}", floor_tiles_csv.join(","))?;
    writeln!(&mut writer, "\t}};")?;

    let ceiling_tiles_csv: Vec<String> = zone.ceiling.iter().map(|t| t.to_string()).collect();
    writeln!(&mut writer, "\tconstexpr uint8_t ceiling_tiles[{}] = {{", zone.width * zone.height * zone.metatile_factor * zone.metatile_factor)?;
    writeln!(&mut writer, "\t\t{}", ceiling_tiles_csv.join(","))?;
    writeln!(&mut writer, "\t}};")?;

    writeln!(&mut writer, "}}")?;

    Ok(())
}

