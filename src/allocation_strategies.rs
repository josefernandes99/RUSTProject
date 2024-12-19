use crate::warehouse::warehouse::{Warehouse, Location};
use crate::warehouse::item::{Item, ItemQuality};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AllocationStrategy {
    Nearest,
    RoundRobin,
}

pub fn nearest_allocation(warehouse: &Warehouse, item: &Item) -> Result<Vec<Location>, String> {
    match item.item_quality {
        ItemQuality::Normal => {
            for row in 0..warehouse.max_dimensions.0 {
                for shelf in 0..warehouse.max_dimensions.1 {
                    for level in 0..warehouse.max_dimensions.2 {
                        for zone in 0..warehouse.max_dimensions.3 {
                            let location = Location { row, shelf, level, zone };
                            if !warehouse.items.contains_key(&location) {
                                return Ok(vec![location]);
                            }
                        }
                    }
                }
            }
            Err("Nenhuma localização disponível encontrada.".to_string())
        },
        ItemQuality::Fragile => {
            for row in 0..warehouse.max_dimensions.0 {
                for shelf in 0..warehouse.max_dimensions.1 {
                    for level in 0..warehouse.max_dimensions.2 {
                        if let Some(nivel_maximo) = item.nivel_maximo {
                            if level > nivel_maximo {
                                continue;
                            }
                        }
                        for zone in 0..warehouse.max_dimensions.3 {
                            let location = Location { row, shelf, level, zone };
                            if !warehouse.items.contains_key(&location) {
                                return Ok(vec![location]);
                            }
                        }
                    }
                }
            }
            Err("Nenhuma localização disponível encontrada para item frágil.".to_string())
        },
        ItemQuality::Oversized => {
            if let Some(required_zones) = item.required_zones {
                let max_zones = warehouse.max_dimensions.3;
                if required_zones > max_zones {
                    return Err("Não há zonas contíguas suficientes para o item oversized.".to_string());
                }
                for row in 0..warehouse.max_dimensions.0 {
                    for shelf in 0..warehouse.max_dimensions.1 {
                        for level in 0..warehouse.max_dimensions.2 {
                            let mut contiguous = Vec::new();
                            for zone in 0..warehouse.max_dimensions.3 {
                                let location = Location { row, shelf, level, zone };
                                if !warehouse.items.contains_key(&location) {
                                    contiguous.push(location.clone());
                                    if contiguous.len() as u32 == required_zones {
                                        return Ok(contiguous);
                                    }
                                } else {
                                    contiguous.clear();
                                }
                            }
                        }
                    }
                }
                Err("Não há zonas contíguas suficientes para o item oversized.".to_string())
            } else {
                Err("Item oversized sem zonas contíguas necessárias especificadas.".to_string())
            }
        },
    }
}

pub fn round_robin_allocation(warehouse: &mut Warehouse, item: &Item) -> Result<Vec<Location>, String> {
    let mut allocated = Vec::new();
    let total_spots = warehouse.max_dimensions.0
        * warehouse.max_dimensions.1
        * warehouse.max_dimensions.2
        * warehouse.max_dimensions.3;

    let mut current = warehouse.last_allocation.clone().unwrap_or(Location { row: 0, shelf: 0, level: 0, zone: 0 });
    let mut attempts = 0;

    loop {
        attempts += 1;
        if attempts > total_spots {
            break; // full scan done
        }

        let mut row = current.row;
        let mut shelf = current.shelf;
        let mut level = current.level;
        let mut zone = current.zone + 1;

        if zone >= warehouse.max_dimensions.3 {
            zone = 0;
            level += 1;
        }
        if level >= warehouse.max_dimensions.2 {
            level = 0;
            shelf += 1;
        }
        if shelf >= warehouse.max_dimensions.1 {
            shelf = 0;
            row += 1;
        }
        if row >= warehouse.max_dimensions.0 {
            row = 0;
        }

        current = Location { row, shelf, level, zone };

        if !warehouse.items.contains_key(&current) {
            match item.item_quality {
                ItemQuality::Normal => {
                    allocated.push(current.clone());
                    warehouse.last_allocation = Some(current);
                    return Ok(allocated);
                },
                ItemQuality::Fragile => {
                    if let Some(nivel_maximo) = item.nivel_maximo {
                        if level > nivel_maximo {
                            continue;
                        }
                    }
                    allocated.push(current.clone());
                    warehouse.last_allocation = Some(current);
                    return Ok(allocated);
                },
                ItemQuality::Oversized => {
                    if let Some(required_zones) = item.required_zones {
                        let max_zones = warehouse.max_dimensions.3;
                        if required_zones > max_zones {
                            continue;
                        }
                        let mut temp_allocated = Vec::new();
                        let mut temp_zone = zone;
                        let mut success = true;
                        for _ in 0..required_zones {
                            if temp_zone >= max_zones {
                                success = false;
                                break;
                            }
                            let temp_location = Location { row, shelf, level, zone: temp_zone };
                            if !warehouse.items.contains_key(&temp_location) {
                                temp_allocated.push(temp_location.clone());
                                temp_zone += 1;
                            } else {
                                success = false;
                                break;
                            }
                        }
                        if success {
                            allocated = temp_allocated;
                            if let Some(last_loc) = allocated.last() {
                                warehouse.last_allocation = Some(last_loc.clone());
                            }
                            return Ok(allocated);
                        }
                    } else {
                        return Err("Item oversized sem zonas contíguas necessárias especificadas.".to_string());
                    }
                },
            }
        }
    }

    Err("Nenhuma localização disponível encontrada.".to_string())
}
