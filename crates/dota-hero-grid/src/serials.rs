//! JSON serialization for [`crate::GridMap`].
//!
//! Internal `Serializable*` types mirror the public API but with
//! serde-compatible field names. Use [`serialize`] to convert a
//! [`crate::GridMap`] into a JSON string.

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

/// Serialize a [`crate::GridMap`] into a JSON string.
///
/// The output contains a `version` field, a list of `configs`, each with
/// `categories` holding position, size, and hero IDs.
///
/// # Example
///
/// ```
/// use dota_hero_grid::{GridMap, Grid, Category, serialize};
///
/// let mut map = GridMap::new();
/// let mut grid = Grid::new("example");
/// grid.add_category(Category::new("str", (0.0, 0.0), (600.0, 400.0)));
/// map.add_grid(grid);
///
/// let json = serialize(&map).unwrap();
/// assert!(json.contains("example"));
/// assert!(json.contains("\"version\":3"));
/// assert!(json.contains("\"x_position\":0.0"));
/// ```
pub fn serialize(gridmap: &crate::GridMap) -> Result<String, serde_json::Error> {
    let sgm: SerializableGridMap = gridmap.clone().into();
    serde_json::to_string(&sgm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Grid, GridMap};

    #[test]
    fn serialize_empty() {
        let gm = GridMap::new();
        let json = serialize(&gm).unwrap();
        assert!(json.contains("\"version\":3"));
        assert!(json.contains("\"configs\":[]"));
    }

    #[test]
    fn serialize_one_grid() {
        let mut gm = GridMap::new();
        let mut grid = Grid::new("test_config");
        grid.add_category(Category::new("str", (0.0, 0.0), (600.0, 400.0)));
        gm.add_grid(grid);

        let json = serialize(&gm).unwrap();
        assert!(json.contains("test_config"));
        assert!(json.contains("str"));
        assert!(json.contains("\"x_position\":0.0"));
        assert!(json.contains("\"y_position\":0.0"));
        assert!(json.contains("\"width\":600.0"));
        assert!(json.contains("\"height\":400.0"));
    }

    #[test]
    fn serialize_multiple_categories() {
        let mut gm = GridMap::new();
        let mut grid = Grid::new("g");
        grid.add_category(Category::new("a", (0.0, 0.0), (100.0, 100.0)));
        grid.add_category(Category::new("b", (100.0, 0.0), (100.0, 100.0)));
        gm.add_grid(grid);

        let json = serialize(&gm).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let cats = &parsed["configs"][0]["categories"];
        assert_eq!(cats.as_array().unwrap().len(), 2);
        assert_eq!(cats[0]["category_name"], "a");
        assert_eq!(cats[1]["category_name"], "b");
    }

    #[test]
    fn serialize_multiple_grids() {
        let mut gm = GridMap::new();
        gm.add_grid(Grid::new("g1"));
        gm.add_grid(Grid::new("g2"));

        let json = serialize(&gm).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["configs"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn serialize_includes_hero_ids() {
        let mut gm = GridMap::new();
        let mut grid = Grid::new("g");
        let mut cat = Category::new("test", (0.0, 0.0), (100.0, 100.0));
        cat.set_hero_ids(vec![1, 42, 99]);
        grid.add_category(cat);
        gm.add_grid(grid);

        let json = serialize(&gm).unwrap();
        assert!(json.contains("[1,42,99]") || json.contains("[1, 42, 99]"));
    }

    #[test]
    fn serialize_roundtrip_version() {
        let gm = GridMap::new();
        let json = serialize(&gm).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["version"], 3);
    }
}