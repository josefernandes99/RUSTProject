use chrono::{NaiveDate, Utc};

#[derive(Hash, PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub enum ItemQuality {
    Fragile,
    Oversized,
    Normal,
}

#[derive(Clone)]
pub struct Item {
    pub(crate) num_id: u32,
    pub name: String,
    pub(crate) item_quantity: u32,
    pub(crate) item_quality: ItemQuality,
    pub(crate) timestamp: i64,
    pub(crate) data_validade: Option<NaiveDate>,
    pub(crate) nivel_maximo: Option<u32>,
    pub(crate) required_zones: Option<u32>,
}

impl Item {
    pub fn new(
        num_id: u32,
        name: String,
        item_quantity: u32,
        item_quality: ItemQuality,
        data_validade: Option<NaiveDate>,
        nivel_maximo: Option<u32>,
        required_zones: Option<u32>,
    ) -> Item {
        let timestamp: i64 = Utc::now().timestamp();
        Item {
            num_id,
            name,
            item_quantity,
            item_quality,
            timestamp,
            data_validade,
            nivel_maximo,
            required_zones,
        }
    }
}
