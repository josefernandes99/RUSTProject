use chrono::NaiveDate;

pub fn parse_date(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%d-%m-%Y").ok()
}

pub fn validate_location_input(row: &str, shelf: &str, level: &str, zone: &str, max_dimensions: (u32, u32, u32, u32)) -> Result<(u32, u32, u32, u32), String> {
    let row_num = row.parse::<u32>().map_err(|_| "Número de fileira inválido.".to_string())?;
    let shelf_num = shelf.parse::<u32>().map_err(|_| "Número de prateleira inválido.".to_string())?;
    let level_num = level.parse::<u32>().map_err(|_| "Número de nível inválido.".to_string())?;
    let zone_num = zone.parse::<u32>().map_err(|_| "Número de zona inválido.".to_string())?;

    if row_num >= max_dimensions.0 || shelf_num >= max_dimensions.1 || level_num >= max_dimensions.2 || zone_num >= max_dimensions.3 {
        return Err("Localização excede as dimensões do armazém.".to_string());
    }

    Ok((row_num, shelf_num, level_num, zone_num))
}
