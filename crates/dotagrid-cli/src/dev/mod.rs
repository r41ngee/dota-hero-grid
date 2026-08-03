use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroPortraitPixels {
    pub id: i64,
    pub rgb: Vec<u8>,
}
