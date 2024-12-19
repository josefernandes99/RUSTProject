use super::item::{Item, ItemQuality};
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone, PartialOrd, Ord)]
pub struct Location {
    pub row: u32,
    pub shelf: u32,
    pub level: u32,
    pub zone: u32,
}

pub struct Warehouse {
    pub items: HashMap<Location, Item>,
    pub max_dimensions: (u32, u32, u32, u32), // (rows, shelves, levels, zones)
    pub usage_count: HashMap<Location, u32>,
}

impl Warehouse {
    pub fn new(max_dimensions: (u32, u32, u32, u32)) -> Self {
        Warehouse {
            items: HashMap::new(),
            max_dimensions,
            usage_count: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<Vec<Location>, String> {
        let allocated_locations = self.find_allocation_spot(&item)?;

        // Check constraints
        match item.item_quality {
            ItemQuality::Fragile => {
                if let Some(nivel_maximo) = item.nivel_maximo {
                    for loc in &allocated_locations {
                        if loc.level > nivel_maximo {
                            return Err(format!("Não é possível armazenar item frágil acima do nível máximo {}", nivel_maximo));
                        }
                    }
                }
            },
            ItemQuality::Oversized => {
                if let Some(required_zones) = item.required_zones {
                    if allocated_locations.len() as u32 != required_zones {
                        return Err(format!("Item oversized requer {} zonas contíguas, mas foram alocadas {}", required_zones, allocated_locations.len()));
                    }
                }
            },
            ItemQuality::Normal => {}
        }

        for loc in &allocated_locations {
            self.items.insert(loc.clone(), item.clone());
            let count = self.usage_count.get(loc).cloned().unwrap_or(0);
            self.usage_count.insert(loc.clone(), count + 1);
        }

        Ok(allocated_locations)
    }

    fn find_allocation_spot(&self, item: &Item) -> Result<Vec<Location>, String> {
        let (rows, shelves, levels, zones) = self.max_dimensions;

        match item.item_quality {
            ItemQuality::Normal => {
                let mut candidates = Vec::new();
                for row in 0..rows {
                    for shelf in 0..shelves {
                        for level in 0..levels {
                            for zone in 0..zones {
                                let loc = Location { row, shelf, level, zone };
                                if !self.items.contains_key(&loc) {
                                    let usage = self.usage_count.get(&loc).cloned().unwrap_or(0);
                                    candidates.push((usage, loc));
                                }
                            }
                        }
                    }
                }

                if candidates.is_empty() {
                    return Err("Nenhuma localização disponível encontrada.".to_string());
                }

                candidates.sort_by(|(u1, l1), (u2, l2)| u1.cmp(u2).then(l1.cmp(l2)));

                Ok(vec![candidates[0].1.clone()])
            },
            ItemQuality::Fragile => {
                let nivel_maximo = item.nivel_maximo.unwrap_or(levels - 1);
                let mut candidates = Vec::new();
                for row in 0..rows {
                    for shelf in 0..shelves {
                        for level in 0..levels {
                            if level > nivel_maximo {
                                continue;
                            }
                            for zone in 0..zones {
                                let loc = Location { row, shelf, level, zone };
                                if !self.items.contains_key(&loc) {
                                    let usage = self.usage_count.get(&loc).cloned().unwrap_or(0);
                                    candidates.push((usage, loc));
                                }
                            }
                        }
                    }
                }

                if candidates.is_empty() {
                    return Err("Nenhuma localização disponível encontrada para item frágil.".to_string());
                }

                candidates.sort_by(|(u1, l1), (u2, l2)| u1.cmp(u2).then(l1.cmp(l2)));

                Ok(vec![candidates[0].1.clone()])
            },
            ItemQuality::Oversized => {
                let required_zones = item.required_zones.ok_or("Item oversized sem zonas contíguas necessárias especificadas.".to_string())?;

                if required_zones == 0 {
                    return Err("Número de zonas contíguas inválido para item oversized.".to_string());
                }

                let (rows, shelves, levels, zones) = self.max_dimensions;
                let mut candidates = Vec::new();
                for row in 0..rows {
                    for shelf in 0..shelves {
                        for level in 0..levels {
                            let mut start_zone = 0;
                            while start_zone + required_zones <= zones {
                                let mut run = Vec::new();
                                let mut sum_usage = 0;
                                let mut all_free = true;
                                for z in start_zone..(start_zone + required_zones) {
                                    let loc = Location { row, shelf, level, zone: z };
                                    if self.items.contains_key(&loc) {
                                        all_free = false;
                                        break;
                                    } else {
                                        let usage = self.usage_count.get(&loc).cloned().unwrap_or(0);
                                        sum_usage += usage;
                                        run.push(loc);
                                    }
                                }

                                if all_free {
                                    candidates.push((sum_usage, run[0].clone(), run));
                                }

                                start_zone += 1;
                            }
                        }
                    }
                }

                if candidates.is_empty() {
                    return Err("Não há zonas contíguas suficientes para o item oversized.".to_string());
                }

                candidates.sort_by(|(sum1, first1, _), (sum2, first2, _)| sum1.cmp(&sum2).then(first1.cmp(&first2)));

                let (_, _, best_run) = candidates[0].clone();
                Ok(best_run)
            },
        }
    }

    pub fn remove_item(&mut self, location: &Location) -> Result<Vec<Item>, String> {
        if let Some(item) = self.items.get(location) {
            let num_id = item.num_id;
            let name = item.name.clone();
            let timestamp = item.timestamp;

            let mut locations_to_remove = Vec::new();
            match item.item_quality {
                ItemQuality::Oversized => {
                    locations_to_remove = self.items
                        .iter()
                        .filter(|(_, itm)| itm.num_id == num_id && itm.name == name && itm.timestamp == timestamp)
                        .map(|(loc, _)| loc.clone())
                        .collect();
                },
                _ => {
                    locations_to_remove.push(location.clone());
                }
            }

            let mut removed_items = Vec::new();
            for loc in &locations_to_remove {
                if let Some(itm) = self.items.remove(loc) {
                    removed_items.push(itm);
                }
            }

            Ok(removed_items)
        } else {
            Err("Nenhum item encontrado nesta localização.".to_string())
        }
    }

    pub fn grouped_items(&self) -> Vec<(Item, Vec<Location>)> {
        let mut map: HashMap<(u32, String, i64), Vec<(Location, Item)>> = HashMap::new();
        for (loc, itm) in &self.items {
            let key = (itm.num_id, itm.name.clone(), itm.timestamp);
            map.entry(key).or_default().push((loc.clone(), itm.clone()));
        }

        let mut result = Vec::new();
        for (_, mut vec_loc_item) in map {
            vec_loc_item.sort_by(|(l1,_), (l2,_)| l1.cmp(l2));
            let (_, first_item) = vec_loc_item[0].clone();
            let locations = vec_loc_item.into_iter().map(|(l,_)| l).collect::<Vec<_>>();
            result.push((first_item, locations));
        }

        result
    }

    pub fn search_by_name(&self, name: &str) -> (bool, u32) {
        let grouped = self.grouped_items();
        let mut total = 0;
        for (itm, _) in grouped {
            if itm.name == name {
                total += itm.item_quantity;
            }
        }
        (total > 0, total)
    }

    pub fn search_by_id(&self, num_id: u32) -> (bool, u32) {
        let grouped = self.grouped_items();
        let mut total = 0;
        for (itm, _) in grouped {
            if itm.num_id == num_id {
                total += itm.item_quantity;
            }
        }
        (total > 0, total)
    }

    pub fn search_locations_by_id(&self, num_id: u32) -> Vec<(Item, Vec<Location>)> {
        let grouped = self.grouped_items();
        let mut result = Vec::new();
        for (itm, locs) in grouped {
            if itm.num_id == num_id {
                result.push((itm, locs));
            }
        }
        result
    }

    pub fn find_expiring_items_by_date(&self, reference_date: NaiveDate) -> Vec<(Item, String, Vec<Location>)> {
        let grouped = self.grouped_items();
        let mut result = Vec::new();
        for (itm, locs) in grouped {
            if let ItemQuality::Fragile = itm.item_quality {
                if let Some(data_validade) = itm.data_validade {
                    if data_validade < reference_date {
                        result.push((itm.clone(), "Expirado".to_string(), locs.clone()));
                    } else {
                        let days_to_expire = (data_validade - reference_date).num_days();
                        if days_to_expire <= 3 && days_to_expire >= 0 {
                            result.push((itm.clone(), format!("Expira em {} dias", days_to_expire), locs.clone()));
                        }
                    }
                }
            }
        }
        result
    }
}
