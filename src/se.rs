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
    writeln!(&mut writer, "namespace sp::{} {{", zone.name)?;

    writeln!(&mut writer, "\tconstexpr uint8_t width() {{ return {}; }}", zone.width * zone.metatile_factor)?;
    writeln!(&mut writer, "\tconstexpr uint8_t height() {{ return {}; }}", zone.height * zone.metatile_factor)?;

    writeln!(&mut writer, "\tconstexpr uint8_t floor_tiles[{}];", zone.width * zone.height * zone.metatile_factor * zone.metatile_factor)?;

    writeln!(&mut writer, "}}")?;

    Ok(())
}

pub fn write_implementation(destination: PathBuf, zone: &Zone) -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(destination)?;
    let mut writer = BufWriter::new(file);

    writeln!(&mut writer, "#include \"sp_{}.h\"", zone.name)?;
    writeln!(&mut writer)?;
    writeln!(&mut writer, "namespace sp::{} {{", zone.name)?;

    let tiles_csv: Vec<String> = zone.floor.iter().map(|t| t.to_string()).collect();
    writeln!(&mut writer, "\tconstexpr uint8_t floor_tiles[{}] = {{", zone.width * zone.height * zone.metatile_factor * zone.metatile_factor)?;
    writeln!(&mut writer, "\t\t{}", tiles_csv.join(","))?;
    writeln!(&mut writer, "}}")?;

    writeln!(&mut writer, "}}")?;
    Ok(())
}
