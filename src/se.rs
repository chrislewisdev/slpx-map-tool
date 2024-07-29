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
    writeln!(&mut writer)?;
    writeln!(&mut writer, "#include \"enemy_type.h\"")?;
    writeln!(&mut writer)?;

    writeln!(&mut writer, "namespace sp {{")?;

    writeln!(&mut writer, "\tclass enemy_spawn {{")?;
    writeln!(&mut writer, "\t\tpublic:")?;
    writeln!(&mut writer, "\t\t\tconst int16_t x;")?;
    writeln!(&mut writer, "\t\t\tconst int16_t y;")?;
    writeln!(&mut writer, "\t\t\tconst sp::enemy_type& enemy_type;")?;
    writeln!(&mut writer, "\t\t\tconstexpr enemy_spawn(uint16_t _x, uint16_t _y, const sp::enemy_type& _enemy_type): x(_x), y(_y), enemy_type(_enemy_type) {{}}")?;
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
    // writeln!(&mut writer, "#include \"portal.h\"")?;
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
    writeln!(
        &mut writer,
        "\tconstexpr int16_t spawn_point_x() {{ return {}; }}",
        zone.player_spawn_point.x
    )?;
    writeln!(
        &mut writer,
        "\tconstexpr int16_t spawn_point_y() {{ return {}; }}",
        zone.player_spawn_point.y
    )?;

    // Floor tiles
    let floor_tiles_csv: Vec<String> = zone.floor.iter().map(|t| t.to_string()).collect();
    writeln!(
        &mut writer,
        "\tconstexpr uint8_t floor_tiles[{}] = {{",
        zone.width * zone.height * zone.metatile_factor * zone.metatile_factor
    )?;
    writeln!(&mut writer, "\t\t{}", floor_tiles_csv.join(","))?;
    writeln!(&mut writer, "\t}};")?;
    writeln!(
        &mut writer,
        "\tconstexpr bn::affine_bg_map_item floor_map(*floor_tiles, bn::size(width(), height()));"
    )?;

    // Ceiling tiles
    let ceiling_tiles_csv: Vec<String> = zone.ceiling.iter().map(|t| t.to_string()).collect();
    writeln!(
        &mut writer,
        "\tconstexpr uint8_t ceiling_tiles[{}] = {{",
        zone.width * zone.height * zone.metatile_factor * zone.metatile_factor
    )?;
    writeln!(&mut writer, "\t\t{}", ceiling_tiles_csv.join(","))?;
    writeln!(&mut writer, "\t}};")?;
    writeln!(
        &mut writer,
        "\tconstexpr bn::affine_bg_map_item ceiling_map(*ceiling_tiles, bn::size(width(), height()));"
    )?;

    // Enemies
    writeln!(&mut writer, "\tconstexpr sp::enemy_spawn enemy_spawns[] = {{")?;
    for enemy in &zone.enemies {
        writeln!(
            &mut writer,
            "\t\tsp::enemy_spawn({}, {}, enemy_type::{}),",
            enemy.spawn_point.x,
            enemy.spawn_point.y,
            enemy.type_id.to_str()
        )?;
    }
    writeln!(&mut writer, "\t}};")?;

    // Portals
    /*if zone.portals.len() > 0 {
        writeln!(&mut writer, "\tconstexpr sp::portal portals[] = {{")?;
        for portal in &zone.portals {
            writeln!(
                &mut writer,
                "\t\tsp::portal(sp::world_zone::{}, {}, {}, {}, {}, {}, {}),",
                portal.target_zone, portal.position.x, portal.position.y, portal.width, portal.height, portal.destination.x, portal.destination.y
            )?;
        }
        writeln!(&mut writer, "\t}};")?;
    }*/

    writeln!(&mut writer, "}}")?;

    Ok(())
}
