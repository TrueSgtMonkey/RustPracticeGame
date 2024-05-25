use bevy::{prelude::*, scene::ron::de::Position};
use crate::components::player::setup_player;
use super::MapBuilder;

pub enum EntityId {
    PLAYER=0,
    ERROR,
}

impl EntityId {
    pub fn get_entity_id_from_usize(id: usize) -> EntityId {
        match id {
            0 => EntityId::PLAYER,
            _ => EntityId::ERROR,
        }
    }
}

pub struct MapEntityPlugin;

impl Plugin for MapEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, map_entity_setup);
    }
}

fn map_entity_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut map_builder: ResMut<MapBuilder>,
) {
    let entities: &Vec<(usize, Vec2)> = &map_builder.entities;

    for (id, mut position) in entities {
        let entity_id: EntityId = EntityId::get_entity_id_from_usize(*id);
        position.x *= 32f32;
        position.y *= -32f32;
        match entity_id {
            EntityId::PLAYER => {
                setup_player(&mut commands, &asset_server, &mut texture_atlas_layouts, &position);
            },
            EntityId::ERROR => {
    
            }
        };
    }


    map_builder.entities.clear();
}
