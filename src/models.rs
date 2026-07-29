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
    pub fn new(name: String) -> Self {
        Self {
            name,
            data: Vec::new(),
        }
    }

    pub fn add_category(&mut self, category: Category) {
        self.data.push(category);
    }
}

#[derive(Getters, Clone)]
pub struct Category {
    name: String,
    pos: (f32, f32),
    size: (f32, f32),
    hero_ids: Vec<u32>,
}

impl Category {
    pub fn new(
        name: String,
        pos: (f32, f32),
        size: (f32, f32)
    ) -> Self {
        Self {
            name,
            pos,
            size,
            hero_ids: Vec::new(),
        }
    }

    pub fn add_hero(&mut self, hero_name: &String) -> Option<()> {
        const HERO_PREFIX: &str = "npc_dota_hero_";
        let true_hero_name = if hero_name.starts_with(HERO_PREFIX) {
            HERO_PREFIX.to_string()
        } else {
            format!("{}{}", HERO_PREFIX, hero_name)
        };

        let hero = rdotaconstants::Hero::get(true_hero_name)?;
        let id = hero.id as u32;
        self.hero_ids.push(id);
        Some(())
    }
}
