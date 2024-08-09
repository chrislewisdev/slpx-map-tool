use crate::model::*;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::PathBuf,
};

pub fn write_zones_header(destination: PathBuf, zone_names: &Vec<String>) -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(destination)?;
    let mut writer = BufWriter::new(file);

    writeln!(&mut writer, "#pragma once")?;
    writeln!(&mut writer)?;
    for zone in zone_names {
        writeln!(&mut writer, "#include \"sp_{}.h\"", zone)?;
    }
    writeln!(&mut writer)?;

    writeln!(&mut writer, "namespace sp {{")?;
    writeln!(&mut writer, "\tconstexpr const world_zone* get_zone_by_name(const bn::string_view& name) {{")?;
    for zone in zone_names {
        let shortname = zone.trim_start_matches("zone_");
        writeln!(&mut writer, "\t\tif (name == \"{}\") return &sp::{}::zone;", shortname, zone)?;
    }
    writeln!(&mut writer)?;
    writeln!(&mut writer, "\t\tBN_ERROR(\"Unrecognised zone:\", name);")?;
    writeln!(&mut writer, "\t\treturn nullptr;")?;
    writeln!(&mut writer, "\t}}")?;
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
    writeln!(&mut writer, "#include \"bn_affine_bg_tiles_items_tiles_v2.h\"")?;
    writeln!(&mut writer)?;
    writeln!(&mut writer, "#include \"vec3.h\"")?;
    writeln!(&mut writer, "#include \"enemy_spawn.h\"")?;
    // writeln!(&mut writer, "#include \"portal.h\"")?;
    writeln!(&mut writer, "#include \"world_zone.h\"")?;
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
    if zone.enemies.len() > 0 {
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
    }

    // Portals
    if zone.portals.len() > 0 {
        writeln!(&mut writer, "\tconstexpr sp::portal portals[] = {{")?;
        for portal in &zone.portals {
            writeln!(
                &mut writer,
                "\t\tsp::portal(\"{}\", {}, {}, {}, {}, {}, {}),",
                portal.target_zone,
                portal.position.x,
                portal.position.y,
                portal.width,
                portal.height,
                portal.destination.x,
                portal.destination.y
            )?;
        }
        writeln!(&mut writer, "\t}};")?;
    }

    writeln!(
        &mut writer,
        "\tconstexpr bn::affine_bg_item floor(bn::affine_bg_tiles_items::tiles_v2, bn::affine_bg_tiles_items::tiles_v2_palette, floor_map);"
    )?;
    writeln!(
        &mut writer,
        "\tconstexpr bn::affine_bg_item ceiling(bn::affine_bg_tiles_items::tiles_v2, bn::affine_bg_tiles_items::tiles_v2_palette, ceiling_map);"
    )?;

    // If there is no portals array, gotta use an empty span instead
    let enemies_ref = if zone.enemies.len() > 0 {
        "enemy_spawns"
    } else {
        "bn::span<const enemy_spawn>()"
    };
    let portals_ref = if zone.portals.len() > 0 {
        "portals"
    } else {
        "bn::span<const portal>()"
    };
    writeln!(
        &mut writer,
        "\tconstexpr world_zone zone(floor, ceiling, vec3(spawn_point_x(), 16, spawn_point_y()), {}, {});",
        enemies_ref, portals_ref
    )?;

    writeln!(&mut writer, "}}")?;

    Ok(())
}
