use std::path;

use bevy::prelude::*;
//use std::vec::Vec;

use super::{super::map_material::MapMaterial, MapBuilder};

enum MapReadMode {
    Header,
    Tiles,
    Collisions,
    Entities,
    Error,
}

impl MapReadMode {
    fn get_map_read_mode_from_str(read_mode_string: &str) -> MapReadMode {
        match read_mode_string {
            "[header]" => MapReadMode::Header,
            "[tiles]" => MapReadMode::Tiles,
            "[collisions]" => MapReadMode::Collisions,
            "[entities]" => MapReadMode::Entities,
            _ => MapReadMode::Error,
        }
    }
}

pub fn parse_map(level_name: &str) -> MapBuilder {
    let mut materials: Vec<String> = Vec::new();
    let mut tile_groups: Vec<(usize, Vec2, Vec2)> = Vec::new();
    let mut collision_groups: Vec<(Vec2, Vec2)> = Vec::new();
    let mut entities: Vec<(usize, Vec2)> = Vec::new();
    let mut read_mode: MapReadMode = MapReadMode::Error;


    for line in std::fs::read_to_string(level_name).unwrap().lines() {
        if line.chars().count() == 0 {
            continue;
        }
        let line = line.trim();

        if line.starts_with("[") {
            read_mode = MapReadMode::get_map_read_mode_from_str(&line);
            continue;
        }

        match read_mode {
            MapReadMode::Header => add_material(&mut materials, &line),
            MapReadMode::Tiles  => add_tile_group(&mut tile_groups, &line),
            MapReadMode::Collisions => add_collision_group(&mut collision_groups, &line),
            MapReadMode::Entities => add_entity(&mut entities, &line),
            MapReadMode::Error => println!("ERROR: Cannot parse: {:?}", line),
        }
    }

    let map_builder: MapBuilder = MapBuilder {
        materials: materials,
        tile_groups: tile_groups,
        collision_groups: collision_groups,
        entities: entities,
        level_name: level_name.to_string(),
    };

    return map_builder;
}

fn add_material(materials: &mut Vec<String>, line: &str) {
    let colon_idx: usize = line.find(":").unwrap();
    let path_side = line[(colon_idx+1)..].trim();
    let path_side = path_side.replace("\"", "");

    materials.push(path_side);
}

fn add_tile_group(tile_groups: &mut Vec<(usize, Vec2, Vec2)>, line: &str) {
    let colon_idx: usize = line.find(":").unwrap();
    let id_side = line[..colon_idx].trim();
    let tuple_side = line[(colon_idx+1)..].trim();

    let semi_colon_idx: usize = tuple_side.find(";").unwrap();
    let min_side = tuple_side[..semi_colon_idx].trim();
    let max_size = tuple_side[(semi_colon_idx+1)..].trim();
    
    let min_vec: Vec2 = get_vec2_from_string(min_side);
    let max_vec: Vec2 = get_vec2_from_string(max_size);
    let id: usize = id_side.parse().expect("Expecting usize!");

    tile_groups.push((id, min_vec,max_vec));
}

fn add_collision_group(collision_groups: &mut Vec<(Vec2, Vec2)>, line: &str) {
    let semi_colon_idx: usize = line.find(";").unwrap();
    let min_side = line[..semi_colon_idx].trim();
    let max_size = line[(semi_colon_idx+1)..].trim();
    
    let min_vec: Vec2 = get_vec2_from_string(min_side);
    let max_vec: Vec2 = get_vec2_from_string(max_size);

    collision_groups.push((min_vec, max_vec));
}

fn add_entity(entities: &mut Vec<(usize, Vec2)>, line: &str) {
    let colon_idx: usize = line.find(":").unwrap();
    let id_side = line[..colon_idx].trim();
    let vec_side = line[(colon_idx+1)..].trim();
    let id: usize  = id_side.parse().expect("Expecting usize!");
    let position = get_vec2_from_string(vec_side);

    entities.push((id, position));
}

fn get_vec2_from_string(line: &str) -> Vec2 {
    let comma_idx: usize = line.find(",").unwrap();
    let closing_parentheses: usize = line.find(")").unwrap();
    let x_min: f32 = line[1..comma_idx].trim().parse().expect("Expecting float!");
    let y_min: f32 = line[comma_idx+1..closing_parentheses].trim().parse().expect("Expecting float!");

    Vec2 {
        x: x_min,
        y: y_min,
    }
}
