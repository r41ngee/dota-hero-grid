use serde::{Deserialize, Serialize};

use crate::Category;

const GRID_VERSION: u32 = 3;

#[derive(Serialize, Deserialize)]
struct SerializableGridMap {
    version: u32,
    configs: Vec<SerializableGrid>,
}

impl From<crate::GridMap> for SerializableGridMap {
    fn from(value: crate::GridMap) -> Self {
        Self {
            version: GRID_VERSION,
            configs: value.data().clone()
                .iter().map(|x| {
                    x.clone().into()
                }).collect()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SerializableGrid {
    config_name: String,
    categories: Vec<SerializableCategory>,
}

impl From<crate::Grid> for SerializableGrid {
    fn from(value: crate::Grid) -> Self {
        let cats = value.data().clone();
        let cats = cats.iter().map(|x| x.clone().into()).collect(); 
        Self {
            config_name: value.name().clone(),
            categories: cats
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SerializableCategory {
    category_name: String,
    x_position: f32,
    y_position: f32,
    width: f32,
    height: f32,
    hero_ids: Vec<u32>,
}

impl From<Category> for SerializableCategory {
    fn from(value: Category) -> Self {
        Self {
            category_name: value.name().clone(),
            x_position: value.pos().0,
            y_position: value.pos().1,
            width: value.size().0,
            height: value.size().1,
            hero_ids: value.hero_ids().clone(),
        }
    }
}

pub fn serialize(gridmap: &crate::GridMap) -> Result<String, serde_json::Error> {
    let sgm: SerializableGridMap = gridmap.clone().into();
    serde_json::to_string(&sgm)
}