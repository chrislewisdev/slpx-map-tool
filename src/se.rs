use crate::model::*;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::PathBuf,
};

pub fn write_enemy_spawn_header(destination: PathBuf) -> anyhow::Result<()> {
    let file = OpenOptions::new()         
        .write(true)
        .create(true)
        .truncate(true)
        .open(destination)?;
    let mut writer = BufWriter::new(file);

    writeln!(&mut writer, "#pragma once")?;
    writeln!(&mut writer)?;
    writeln!(&mut writer, "#include \"bn_core.h\"")?;
    writeln!(&mut writer, "#include \"bn_string_view.h\"")?;
    writeln!(&mut writer)?;

    writeln!(&mut writer, "namespace sp {{")?;

    writeln!(&mut writer, "\tclass enemy_spawn {{")?;
    writeln!(&mut writer, "\t\tpublic:")?;
    writeln!(&mut writer, "\t\t\tconst int16_t x;")?;
    writeln!(&mut writer, "\t\t\tconst int16_t y;")?;
    writeln!(&mut writer, "\t\t\tconst bn::string_view type_id;")?;
    writeln!(&mut writer, "\t\t\tconstexpr enemy_spawn(uint16_t _x, uint16_t _y, const bn::string_view& _type_id): x(_x), y(_y), type_id(_type_id) {{}}")?;
    writeln!(&mut writer, "\t}};")?;

    writeln!(&mut writer, "}}")?;

    Ok(())
}

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
    writeln!(&mut writer, "#include \"bn_size.h\"")?;
    writeln!(&mut writer, "#include \"bn_span.h\"")?;
    writeln!(&mut writer, "#include \"bn_affine_bg_map_item.h\"")?;
    writeln!(&mut writer)?;
    writeln!(&mut writer, "#include \"sp_enemy_spawn.h\"")?;
    writeln!(&mut writer)?;
    writeln!(&mut writer, "namespace sp::{} {{", zone.name)?;

    // Width/height info
    writeln!(
        &mut writer,
        "\tconstexpr uint8_t width() {{ return {}; }}",
        zone.width * zone.metatile_factor
    )?;
    writeln!(
        &mut writer,
        "\tconstexpr uint8_t height() {{ return {}; }}",
        zone.height * zone.metatile_factor
    )?;

    // Spawn point
    writeln!(&mut writer, "\tconstexpr int16_t spawn_point_x() {{ return {}; }}", zone.player_spawn_point.x)?;
    writeln!(&mut writer, "\tconstexpr int16_t spawn_point_y() {{ return {}; }}", zone.player_spawn_point.y)?;

    // Floor tiles
    let floor_tiles_csv: Vec<String> = zone.floor.iter().map(|t| t.to_string()).collect();
    writeln!(
        &mut writer,
        "\tconstexpr uint8_t floor_tiles[{}] = {{",
        zone.width * zone.height * zone.metatile_factor * zone.metatile_factor
    )?;
    writeln!(&mut writer, "\t\t{}", floor_tiles_csv.join(","))?;
    writeln!(&mut writer, "\t}};")?;
    writeln!(&mut writer, "\tconstexpr bn::affine_bg_map_item floor_map(*floor_tiles, bn::size(width(), height()));")?;

    // Ceiling tiles
    let ceiling_tiles_csv: Vec<String> = zone.ceiling.iter().map(|t| t.to_string()).collect();
    writeln!(
        &mut writer,
        "\tconstexpr uint8_t ceiling_tiles[{}] = {{",
        zone.width * zone.height * zone.metatile_factor * zone.metatile_factor
    )?;
    writeln!(&mut writer, "\t\t{}", ceiling_tiles_csv.join(","))?;
    writeln!(&mut writer, "\t}};")?;
    writeln!(&mut writer, "\tconstexpr bn::affine_bg_map_item ceiling_map(*ceiling_tiles, bn::size(width(), height()));")?;

    // Enemies
    writeln!(&mut writer, "\tconstexpr sp::enemy_spawn enemy_spawns[] = {{")?;
    for enemy in &zone.enemies {
        // TODO: Fix the spawn point co-ordinate space!!
        writeln!(&mut writer, "\t\tsp::enemy_spawn({}, {}, \"basic\"),", enemy.spawn_point.x - 256, 256 - enemy.spawn_point.y)?;
    }
    writeln!(&mut writer, "\t}};")?;

    writeln!(&mut writer, "}}")?;

    Ok(())
}
