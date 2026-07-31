use serde::{Serialize, Deserialize};
use dota_hero_grid::*;

#[derive(Serialize, Deserialize)]
pub struct SGridMap {
    grids: Vec<SGrid>,
}

impl Into<GridMap> for SGridMap {
    fn into(self) -> GridMap {
        let mut gm = GridMap::new();
        for g in self.grids {
            gm.add_grid(g.into());
        }

        gm
    }
}

#[derive(Serialize, Deserialize)]
pub struct SGrid {
    name: String,
    categories: Vec<SCategory>,
}

impl Into<Grid> for SGrid {
    fn into(self) -> Grid {
        let mut grid = Grid::new(&self.name);
        for cat in self.categories {
            grid.add_category(cat.into());
        }

        grid
    }
}

#[derive(Serialize, Deserialize)]
pub struct SCategory {
    name: String,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    heroes: Vec<String>,
}

impl Into<Category> for SCategory {
    fn into(self) -> Category {
        let mut cat = Category::new(&self.name, (self.x, self.y), (self.w, self.h));
        for hero in self.heroes.iter() {
            cat.add_hero(hero);
        }

        cat
    }
}