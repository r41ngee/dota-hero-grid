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
}
