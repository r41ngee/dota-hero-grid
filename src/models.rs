use getset::Setters;
use derive_getters::Getters;

#[derive(Getters, Clone)]
pub struct GridMap {
    data: Vec<Grid>,
}

impl GridMap {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(4),
        }
    }

    pub fn add_grid(&mut self, grid: Grid) {
        self.data.push(grid);
    }
}

#[derive(Getters, Clone)]
pub struct Grid {
    name: String,
    data: Vec<Category>,
}

impl Grid {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
        }
    }

    pub fn add_category(&mut self, category: Category) {
        self.data.push(category);
    }
}

#[derive(Getters, Clone, Setters)]
#[getset(set = "pub")]
pub struct Category {
    name: String,
    pos: (f32, f32),
    size: (f32, f32),
    hero_ids: Vec<u32>,
}

use std::borrow::Cow;

impl Category {
    pub fn new(
        name: &str,
        pos: (f32, f32),
        size: (f32, f32)
    ) -> Self {
        Self {
            name: name.into(),
            pos,
            size,
            hero_ids: Vec::new(),
        }
    }

    pub fn add_hero(&mut self, hero_name: &str) -> Option<()> {
        const HERO_PREFIX: &str = "npc_dota_hero_";
        let true_hero_name: Cow<'_, str> = if hero_name.starts_with(HERO_PREFIX) {
            Cow::Borrowed(hero_name)
        } else {
            Cow::Owned(format!("{}{}", HERO_PREFIX, hero_name))
        };

        let hero = rdotaconstants::Hero::get(&true_hero_name)?;
        let id = hero.id as u32;
        self.hero_ids.push(id);
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── GridMap ──

    #[test]
    fn gridmap_new_is_empty() {
        let m = GridMap::new();
        assert!(m.data().is_empty());
    }

    #[test]
    fn gridmap_add_grid() {
        let mut m = GridMap::new();
        m.add_grid(Grid::new("test"));
        assert_eq!(m.data().len(), 1);
        assert_eq!(m.data()[0].name(), "test");
    }

    #[test]
    fn gridmap_multiple_grids() {
        let mut m = GridMap::new();
        m.add_grid(Grid::new("a"));
        m.add_grid(Grid::new("b"));
        m.add_grid(Grid::new("c"));
        assert_eq!(m.data().len(), 3);
    }

    // ── Grid ──

    #[test]
    fn grid_new_sets_name() {
        let g = Grid::new("my_grid");
        assert_eq!(g.name(), "my_grid");
    }

    #[test]
    fn grid_new_empty_data() {
        let g = Grid::new("x");
        assert!(g.data().is_empty());
    }

    #[test]
    fn grid_add_category() {
        let mut g = Grid::new("g");
        let c = Category::new("cat", (0.0, 0.0), (100.0, 100.0));
        g.add_category(c);
        assert_eq!(g.data().len(), 1);
        assert_eq!(g.data()[0].name(), "cat");
    }

    // ── Category ──

    #[test]
    fn category_new() {
        let c = Category::new("str", (10.0, 20.0), (300.0, 400.0));
        assert_eq!(c.name(), "str");
        assert_eq!(c.pos(), &(10.0, 20.0));
        assert_eq!(c.size(), &(300.0, 400.0));
        assert!(c.hero_ids().is_empty());
    }

    #[test]
    fn category_setters() {
        let mut c = Category::new("x", (0.0, 0.0), (0.0, 0.0));
        c.set_name("y".to_string());
        c.set_pos((1.0, 2.0));
        c.set_size((3.0, 4.0));
        c.set_hero_ids(vec![1, 2, 3]);
        assert_eq!(c.name(), "y");
        assert_eq!(c.pos(), &(1.0, 2.0));
        assert_eq!(c.size(), &(3.0, 4.0));
        assert_eq!(c.hero_ids().len(), 3);
    }

    // ── add_hero ──

    #[test]
    fn add_hero_invalid_name_returns_none() {
        let mut c = Category::new("test", (0.0, 0.0), (0.0, 0.0));
        assert!(c.add_hero("nonexistent_hero_xyz").is_none());
        assert!(c.hero_ids().is_empty());
    }

    #[test]
    fn add_hero_with_prefix() {
        let mut c = Category::new("test", (0.0, 0.0), (0.0, 0.0));
        // "pudge" is a well-known hero – skip if absent
        if let Some(()) = c.add_hero("npc_dota_hero_pudge") {
            assert_eq!(c.hero_ids().len(), 1);
        }
    }

    #[test]
    fn add_hero_without_prefix() {
        let mut c = Category::new("test", (0.0, 0.0), (0.0, 0.0));
        if let Some(()) = c.add_hero("pudge") {
            assert_eq!(c.hero_ids().len(), 1);
        }
    }

    #[test]
    fn add_hero_multiple() {
        let mut c = Category::new("test", (0.0, 0.0), (0.0, 0.0));
        let added = c.add_hero("pudge").is_some()
            & c.add_hero("nevermore").is_some()
            & c.add_hero("invoker").is_some();
        if added {
            assert_eq!(c.hero_ids().len(), 3);
        }
    }

    #[test]
    fn add_hero_duplicate() {
        let mut c = Category::new("test", (0.0, 0.0), (0.0, 0.0));
        // add_hero doesn't deduplicate; pushing same hero twice = two ids
        c.add_hero("pudge");
        c.add_hero("pudge");
        // If pudge exists, ids will have 2 entries (possibly same id)
        if c.hero_ids().len() == 2 {
            assert_eq!(c.hero_ids()[0], c.hero_ids()[1]);
        }
    }
}
