mod warehouse;
mod utils;

use eframe::egui;
use warehouse::item::{Item, ItemQuality};
use warehouse::warehouse::{Location, Warehouse};
use utils::{parse_date, validate_location_input};

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

const WAREHOUSE_DIMENSIONS: (u32, u32, u32, u32) = (5, 5, 5, 5);

struct App {
    warehouse: Warehouse,
    current_action: String,
    add_item_form: AddItemForm,
    search_by_id_form: SearchByIdForm,
    search_by_name_form: SearchByNameForm,
    remove_item_form: RemoveItemForm,
    search_results: String,
    logs: Vec<String>,
    show_name_popup: bool,
    search_location_by_id_form: SearchLocationByIdForm,
    check_expiring_form: CheckExpiringForm,
    next_id: u32,
    grid_scale: f32,

    // New mapping from name to ID
    name_to_id: HashMap<String, u32>,
}

struct AddItemForm {
    name: String,
    quantity: String,
    quality: ItemQuality,
    data_validade: String, // DD-MM-YYYY
    nivel_maximo: String,
    required_zones: String,
}

struct SearchByIdForm {
    id: String,
}

struct SearchByNameForm {
    name: String,
}

struct RemoveItemForm {
    row: String,
    shelf: String,
    level: String,
    zone: String,
}

struct SearchLocationByIdForm {
    id: String,
}

struct CheckExpiringForm {
    date: String, // DD-MM-YYYY
}

impl App {
    fn new(max_dimensions: (u32,u32,u32,u32)) -> Self {
        Self {
            warehouse: Warehouse::new(max_dimensions),
            current_action: "Bem-vindo ao Gestor de Armazém!".to_string(),
            add_item_form: AddItemForm {
                name: "".to_string(),
                quantity: "".to_string(),
                quality: ItemQuality::Normal,
                data_validade: "".to_string(),
                nivel_maximo: "".to_string(),
                required_zones: "".to_string(),
            },
            search_by_id_form: SearchByIdForm { id: "".to_string() },
            search_by_name_form: SearchByNameForm { name: "".to_string() },
            remove_item_form: RemoveItemForm {
                row: "".to_string(),
                shelf: "".to_string(),
                level: "".to_string(),
                zone: "".to_string(),
            },
            search_results: "".to_string(),
            logs: Vec::new(),
            show_name_popup: false,
            search_location_by_id_form: SearchLocationByIdForm { id: "".to_string() },
            check_expiring_form: CheckExpiringForm { date: "".to_string() },
            next_id: 1,
            grid_scale: 1.0,
            name_to_id: HashMap::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Left Side Panel with logs
        egui::SidePanel::left("side_panel").resizable(true).show(ctx, |ui| {
            ui.separator();
            if ui.button("Adicionar Item").clicked() {
                self.current_action = "Adding Item".to_string();
            }
            if ui.button("Buscar Item por ID").clicked() {
                self.current_action = "Search by ID".to_string();
            }
            if ui.button("Buscar Item por Nome").clicked() {
                self.current_action = "Search by Name".to_string();
            }
            if ui.button("Buscar Local de Item por ID").clicked() {
                self.current_action = "Search Location by ID".to_string();
            }
            if ui.button("Listar todos os Itens").clicked() {
                self.current_action = "Listing All Items".to_string();
                self.search_results = self.list_items();
                self.logs.push("--------------------------------------".to_string());
                self.logs.push("Listando todos os itens:".to_string());
                self.logs.push("--------------------------------------".to_string());
                self.logs.push(self.search_results.clone());
            }
            if ui.button("Verificar Expirados por Data").clicked() {
                self.current_action = "Checking Expiring Items by Date".to_string();
            }
            if ui.button("Remover Item").clicked() {
                self.current_action = "Removing Item".to_string();
            }

            ui.separator();
            ui.heading("Logs:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for line in &self.logs {
                    ui.monospace(line);
                }
            });
        });

        // Right Side Panel for Legend (resizable)
        egui::SidePanel::right("legend_panel")
            .resizable(true)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.heading("Legenda:");
                self.render_legend(ui);
            });

        // Central Panel for main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();

            match self.current_action.as_str() {
                "Adding Item" => self.render_add_item(ui),
                "Search by ID" => self.render_search_by_id(ui),
                "Search by Name" => self.render_search_by_name(ui),
                "Listing All Items" => {
                    ui.label("Verifique o log no painel lateral para a lista de todos os itens.");
                }
                "Checking Expiring Items by Date" => self.render_check_expiring_by_date(ui),
                "Removing Item" => self.render_remove_item(ui),
                "Search Location by ID" => self.render_search_location_by_id(ui),
                _ => {
                    ui.label(&self.current_action);
                }
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.heading("Armazém:");
                if ui.button("Zoom In").clicked() {
                    self.grid_scale *= 1.05;
                }
                if ui.button("Zoom Out").clicked() {
                    self.grid_scale /= 1.05;
                    if self.grid_scale < 0.1 {
                        self.grid_scale = 0.1;
                    }
                }
            });

            self.render_warehouse(ui);
        });

        // Popup for name selection
        if self.show_name_popup {
            let id = egui::Id::new("name_popup");
            egui::Window::new("Items no Armazém")
                .id(id)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Escolha um item:");
                    let names: Vec<_> = self.warehouse.items.values().map(|i| i.name.clone()).collect();
                    let mut unique_names = names.clone();
                    unique_names.sort();
                    unique_names.dedup();
                    for n in unique_names {
                        if ui.button(&n).clicked() {
                            if let Some(item) = self.warehouse.items.values().find(|it| it.name == n) {
                                self.add_item_form.name = item.name.clone();
                                self.add_item_form.quantity = item.item_quantity.to_string();
                                self.add_item_form.quality = item.item_quality.clone();
                                if let ItemQuality::Fragile = item.item_quality {
                                    if let Some(dv) = item.data_validade {
                                        self.add_item_form.data_validade = dv.format("%d-%m-%Y").to_string();
                                    }
                                    if let Some(nm) = item.nivel_maximo {
                                        self.add_item_form.nivel_maximo = nm.to_string();
                                    }
                                } else if let ItemQuality::Oversized = item.item_quality {
                                    if let Some(rz) = item.required_zones {
                                        self.add_item_form.required_zones = rz.to_string();
                                    }
                                }
                            }
                            self.show_name_popup = false;
                        }
                    }
                    if ui.button("Fechar").clicked() {
                        self.show_name_popup = false;
                    }
                });
        }
    }
}

impl App {
    fn render_add_item(&mut self, ui: &mut egui::Ui) {
        ui.label("Insira os detalhes do item:");

        ui.horizontal(|ui| {
            ui.label("Nome:");
            let response = ui.text_edit_singleline(&mut self.add_item_form.name);
            if response.clicked() {
                self.show_name_popup = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Quantidade:");
            ui.text_edit_singleline(&mut self.add_item_form.quantity);
        });

        ui.horizontal(|ui| {
            ui.label("Qualidade:");
            egui::ComboBox::from_label("")
                .selected_text(match self.add_item_form.quality {
                    ItemQuality::Fragile => "Fragile",
                    ItemQuality::Oversized => "Oversized",
                    ItemQuality::Normal => "Normal",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.add_item_form.quality, ItemQuality::Fragile, "Fragile");
                    ui.selectable_value(&mut self.add_item_form.quality, ItemQuality::Oversized, "Oversized");
                    ui.selectable_value(&mut self.add_item_form.quality, ItemQuality::Normal, "Normal");
                });
        });

        if let ItemQuality::Fragile = self.add_item_form.quality {
            ui.horizontal(|ui| {
                ui.label("Data de Validade (DD-MM-YYYY):");
                ui.text_edit_singleline(&mut self.add_item_form.data_validade);
            });
            ui.horizontal(|ui| {
                ui.label("Nível Máximo de Armazenamento:");
                ui.text_edit_singleline(&mut self.add_item_form.nivel_maximo);
            });
        } else if let ItemQuality::Oversized = self.add_item_form.quality {
            ui.horizontal(|ui| {
                ui.label("Zonas Contíguas Necessárias:");
                ui.text_edit_singleline(&mut self.add_item_form.required_zones);
            });
        }

        if ui.button("Adicionar").clicked() {
            match self.create_item_from_form() {
                Ok(mut item) => {
                    // If the name doesn't exist in name_to_id, assign next_id and increment.
                    // If it exists, use that ID.
                    let item_id = if let Some(&existing_id) = self.name_to_id.get(&item.name) {
                        existing_id
                    } else {
                        let new_id = self.next_id;
                        self.next_id += 1;
                        self.name_to_id.insert(item.name.clone(), new_id);
                        new_id
                    };
                    item.num_id = item_id;

                    match self.warehouse.add_item(item) {
                        Ok(locations) => {
                            self.logs.push("--------------------------------------".to_string());
                            self.logs.push("Item adicionado com sucesso!".to_string());
                            self.logs.push("Localizações:".to_string());
                            for loc in &locations {
                                self.logs.push(format!("   (F{}, P{}, N{}, Z{})", loc.row, loc.shelf, loc.level, loc.zone));
                            }
                            self.logs.push("--------------------------------------".to_string());
                            self.current_action = "Item Adicionado".to_string();
                        },
                        Err(err) => {
                            self.logs.push(format!("Erro: {}", err));
                            self.current_action = "Erro ao Adicionar".to_string();
                        }
                    }
                },
                Err(err) => {
                    self.logs.push(format!("Erro: {}", err));
                    self.current_action = "Erro ao Adicionar".to_string();
                }
            }
        }
    }

    fn render_search_by_id(&mut self, ui: &mut egui::Ui) {
        ui.label("Insira o ID do Item:");
        ui.text_edit_singleline(&mut self.search_by_id_form.id);
        if ui.button("Buscar").clicked() {
            if let Ok(num_id) = self.search_by_id_form.id.trim().parse::<u32>() {
                let (found, count) = self.warehouse.search_by_id(num_id);
                self.logs.push("--------------------------------------".to_string());
                self.logs.push(format!("Resultado da busca por ID {}:", num_id));
                if found {
                    self.logs.push(format!("   Encontrado(s) {} item(ns) com ID {}", count, num_id));
                } else {
                    self.logs.push(format!("   Nenhum item encontrado com ID {}", num_id));
                }
                self.logs.push("--------------------------------------".to_string());
                self.current_action = "Busca por ID Concluída".to_string();
            } else {
                let msg = "Formato de ID inválido.".to_string();
                self.logs.push(msg);
                self.current_action = "Erro na Busca por ID".to_string();
            }
        }
    }

    fn render_search_by_name(&mut self, ui: &mut egui::Ui) {
        ui.label("Insira o Nome do Item:");
        ui.text_edit_singleline(&mut self.search_by_name_form.name);
        if ui.button("Buscar").clicked() {
            let name = self.search_by_name_form.name.trim();
            if !name.is_empty() {
                let (found, count) = self.warehouse.search_by_name(name);
                self.logs.push("--------------------------------------".to_string());
                self.logs.push(format!("Resultado da busca por nome '{}':", name));
                if found {
                    self.logs.push(format!("   Encontrado(s) {} item(ns) com o nome '{}'", count, name));
                } else {
                    self.logs.push(format!("   Nenhum item encontrado chamado '{}'", name));
                }
                self.logs.push("--------------------------------------".to_string());
                self.current_action = "Busca por Nome Concluída".to_string();
            } else {
                let msg = "Nome não pode estar vazio.".to_string();
                self.logs.push(msg);
                self.current_action = "Erro na Busca por Nome".to_string();
            }
        }
    }

    fn render_search_location_by_id(&mut self, ui: &mut egui::Ui) {
        ui.label("Insira o ID do Item para ver as localizações:");
        ui.text_edit_singleline(&mut self.search_location_by_id_form.id);
        if ui.button("Buscar").clicked() {
            if let Ok(num_id) = self.search_location_by_id_form.id.trim().parse::<u32>() {
                let results = self.warehouse.search_locations_by_id(num_id);
                self.logs.push("--------------------------------------".to_string());
                self.logs.push(format!("Localizações para o ID {}:", num_id));
                if results.is_empty() {
                    self.logs.push("   Nenhum item encontrado.".to_string());
                } else {
                    for (itm, locs) in results {
                        self.logs.push(format!("   ID: {}, Nome: {}, Quantidade: {}, Qualidade: {:?}", itm.num_id, itm.name, itm.item_quantity, itm.item_quality));
                        self.logs.push("   Localizações:".to_string());
                        for loc in locs {
                            self.logs.push(format!("      (F{},P{},N{},Z{})", loc.row, loc.shelf, loc.level, loc.zone));
                        }
                    }
                }
                self.logs.push("--------------------------------------".to_string());
                self.current_action = "Busca por Localizações Concluída".to_string();
            } else {
                let msg = "Formato de ID inválido.".to_string();
                self.logs.push(msg);
                self.current_action = "Erro na Busca por Localizações por ID".to_string();
            }
        }
    }

    fn render_remove_item(&mut self, ui: &mut egui::Ui) {
        ui.label("Insira a Localização para Remover o Item:");

        ui.horizontal(|ui| {
            ui.label("Fileira:");
            ui.text_edit_singleline(&mut self.remove_item_form.row);
            ui.label("Prateleira:");
            ui.text_edit_singleline(&mut self.remove_item_form.shelf);
        });
        ui.horizontal(|ui| {
            ui.label("Nível:");
            ui.text_edit_singleline(&mut self.remove_item_form.level);
            ui.label("Zona:");
            ui.text_edit_singleline(&mut self.remove_item_form.zone);
        });

        if ui.button("Remover Item").clicked() {
            match validate_location_input(&self.remove_item_form.row, &self.remove_item_form.shelf, &self.remove_item_form.level, &self.remove_item_form.zone, self.warehouse.max_dimensions) {
                Ok((row_num, shelf_num, level_num, zone_num)) => {
                    let location = Location { row: row_num, shelf: shelf_num, level: level_num, zone: zone_num };
                    match self.warehouse.remove_item(&location) {
                        Ok(items) => {
                            self.logs.push("--------------------------------------".to_string());
                            if !items.is_empty() {
                                self.logs.push("Remoção realizada com sucesso!".to_string());
                                for i in items {
                                    self.logs.push(format!("   Item removido: ID={}, Nome={}, Quantidade={}", i.num_id, i.name, i.item_quantity));
                                }
                            } else {
                                self.logs.push("Nenhum item removido.".to_string());
                            }
                            self.logs.push("--------------------------------------".to_string());
                            self.current_action = "Remoção Concluída".to_string();
                        },
                        Err(err) => {
                            let msg = format!("Erro: {}", err);
                            self.logs.push(msg);
                            self.current_action = "Erro na Remoção".to_string();
                        }
                    }
                },
                Err(err_msg) => {
                    self.logs.push(format!("Erro: {}", err_msg));
                    self.current_action = "Erro na Remoção".to_string();
                }
            }
        }
    }

    fn render_check_expiring_by_date(&mut self, ui: &mut egui::Ui) {
        ui.label("Insira a data de referência (DD-MM-YYYY):");
        ui.text_edit_singleline(&mut self.check_expiring_form.date);

        if ui.button("Verificar").clicked() {
            if let Some(ref_date) = parse_date(&self.check_expiring_form.date) {
                let results = self.warehouse.find_expiring_items_by_date(ref_date);
                self.logs.push("--------------------------------------".to_string());
                self.logs.push(format!("A verificar itens a partir da data {}:", ref_date.format("%d-%m-%Y")));
                if results.is_empty() {
                    self.logs.push("   Nenhum item expirado ou próximo de expirar.".to_string());
                } else {
                    for (itm, status, locs) in results {
                        self.logs.push("--------------------------------------".to_string());
                        self.logs.push(format!("ID: {}, Nome: {}, Status: {}", itm.num_id, itm.name, status));
                        self.logs.push("Localizações:".to_string());
                        for loc in locs {
                            self.logs.push(format!("   (F{},P{},N{},Z{})", loc.row, loc.shelf, loc.level, loc.zone));
                        }
                    }
                }
                self.logs.push("--------------------------------------".to_string());
                self.current_action = "Verificação de Validade Concluída".to_string();
            } else {
                let msg = "Data inválida. Use DD-MM-YYYY.".to_string();
                self.logs.push(msg);
                self.current_action = "Erro na Verificação de Validade".to_string();
            }
        }
    }

    fn list_items(&self) -> String {
        let mut grouped = self.warehouse.grouped_items();
        grouped.sort_by(|(a,_),(b,_)| a.name.cmp(&b.name));

        let mut items_strs = Vec::new();
        for (item, locs) in grouped {
            let date_str = if let Some(dv) = item.data_validade {
                dv.format("%d-%m-%Y").to_string()
            } else {
                "N/A".to_string()
            };
            let extra = match item.item_quality {
                ItemQuality::Fragile => format!("Validade: {}, Nível Máximo: {:?}", date_str, item.nivel_maximo),
                ItemQuality::Oversized => format!("Zonas Necessárias: {:?}", item.required_zones),
                ItemQuality::Normal => "N/A".to_string(),
            };

            items_strs.push(format!(
                "Nome: {}, ID: {}, Quantidade: {}, Qualidade: {:?}, Timestamp: {}, {}\n   Localizações: {}",
                item.name,
                item.num_id,
                item.item_quantity,
                item.item_quality,
                item.timestamp,
                extra,
                locs.iter().map(|l| format!("(F{},P{},N{},Z{})", l.row, l.shelf, l.level, l.zone)).collect::<Vec<_>>().join(", ")
            ));
        }
        items_strs.join("\n--------------------------------------\n")
    }

    fn create_item_from_form(&self) -> Result<Item, String> {
        let name = self.add_item_form.name.trim().to_string();
        if name.is_empty() {
            return Err("Nome não pode estar vazio.".to_string());
        }
        let quantity = self.add_item_form.quantity.trim().parse::<u32>().map_err(|_| "Quantidade inválida.".to_string())?;
        let item_quality = self.add_item_form.quality.clone();

        let mut data_validade = None;
        let mut nivel_maximo = None;
        let mut required_zones = None;

        match item_quality {
            ItemQuality::Fragile => {
                data_validade = Some(parse_date(&self.add_item_form.data_validade).ok_or("Formato de data inválido. Use DD-MM-YYYY.")?);
                nivel_maximo = Some(self.add_item_form.nivel_maximo.trim().parse::<u32>().map_err(|_| "Nível máximo inválido.".to_string())?);
            },
            ItemQuality::Oversized => {
                required_zones = Some(self.add_item_form.required_zones.trim().parse::<u32>().map_err(|_| "Número de zonas contíguas inválido.".to_string())?);
            },
            ItemQuality::Normal => {},
        }

        Ok(Item::new(
            0,
            name,
            quantity,
            item_quality,
            data_validade,
            nivel_maximo,
            required_zones,
        ))
    }

    fn render_warehouse(&mut self, ui: &mut egui::Ui) {
        let row_spacing = 40.0;
        let shelf_spacing = 20.0;
        let level_spacing = 2.0;
        let zone_spacing = 1.0;

        let (rows, shelves, levels, zones) = self.warehouse.max_dimensions;

        let available_rect = ui.available_rect_before_wrap();
        ui.allocate_rect(available_rect, egui::Sense::hover());

        let width = available_rect.width();
        let height = available_rect.height();

        let shelves_float = shelves as f32;
        let zones_float = zones as f32;
        let rows_float = rows as f32;
        let levels_float = levels as f32;

        let cell_size_h = if shelves > 0 && zones > 0 {
            (width - (shelves_float - 1.0)*shelf_spacing - shelves_float*(zones_float - 1.0)*zone_spacing) / (shelves_float * zones_float)
        } else {
            20.0
        };

        let cell_size_v = if rows > 0 && levels > 0 {
            (height - (rows_float - 1.0)*row_spacing - rows_float*(levels_float - 1.0)*level_spacing) / (rows_float * levels_float)
        } else {
            20.0
        };

        let base_cell_size = cell_size_h.min(cell_size_v).max(1.0);
        let cell_size = base_cell_size * self.grid_scale;

        let painter = ui.painter();

        for row in 0..rows {
            let mut y = available_rect.min.y;
            for _ in 0..row {
                y += row_spacing * self.grid_scale;
                y += (levels_float * base_cell_size + (levels_float - 1.0)*level_spacing) * self.grid_scale;
            }

            for level in 0..levels {
                let mut yy = y;
                if level > 0 {
                    yy += level_spacing * (level as f32);
                }
                yy += cell_size * (level as f32);

                for shelf in 0..shelves {
                    let mut x = available_rect.min.x;
                    if shelf > 0 {
                        x += shelf_spacing * (shelf as f32);
                    }
                    x += (shelf as f32)*zones_float*(cell_size + zone_spacing);

                    for zone in 0..zones {
                        let xx = x + (zone as f32)*(cell_size + zone_spacing);

                        let cell_rect = egui::Rect::from_min_size(
                            egui::pos2(xx, yy),
                            egui::vec2(cell_size, cell_size)
                        );

                        let location = Location { row, shelf, level, zone };
                        let item = self.warehouse.items.get(&location);

                        let color = if let Some(item) = item {
                            self.color_for_item(item)
                        } else {
                            egui::Color32::from_gray(180)
                        };

                        painter.rect_filled(cell_rect, 0.0, color);
                    }
                }
            }
        }
    }

    fn render_legend(&self, ui: &mut egui::Ui) {
        let mut item_map = HashMap::new();
        for item in self.warehouse.items.values() {
            let color = self.color_for_quality_name(item.item_quality.clone(), &item.name);
            item_map.entry((item.item_quality.clone(), item.name.clone())).or_insert(color);
        }

        // Replace "Empty (Normal)" with just "Empty"
        item_map.entry((ItemQuality::Normal, "Empty".to_string())).or_insert(egui::Color32::from_gray(180));

        let mut entries: Vec<_> = item_map.into_iter().collect();
        entries.sort_by(|((q1, n1), _), ((q2, n2), _)| {
            n1.cmp(n2).then(q1.cmp(q2))
        });

        for ((quality, name), color) in entries {
            let label_text = if name == "Empty" { "Empty".to_string() } else { format!("{} ({:?})", name, quality) };
            ui.horizontal(|ui| {
                let rect = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover()).0;
                ui.painter().rect_filled(rect, 0.0, color);
                ui.label(label_text);
            });
        }
    }

    fn color_for_item(&self, item: &Item) -> egui::Color32 {
        self.color_for_quality_name(item.item_quality.clone(), &item.name)
    }

    fn color_for_quality_name(&self, q: ItemQuality, name: &str) -> egui::Color32 {
        let mut hasher = DefaultHasher::new();
        q.hash(&mut hasher);
        name.hash(&mut hasher);
        let hash = hasher.finish();
        let hue = (hash % 360) as f32;
        self.hsv_to_color32(hue, 1.0, 1.0)
    }

    fn hsv_to_color32(&self, h: f32, s: f32, v: f32) -> egui::Color32 {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let r_ = ((r + m) * 255.0) as u8;
        let g_ = ((g + m) * 255.0) as u8;
        let b_ = ((b + m) * 255.0) as u8;
        egui::Color32::from_rgb(r_, g_, b_)
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gestor de Armazenamento v1.0",
        options,
        Box::new(|_cc| Ok(Box::new(App::new(WAREHOUSE_DIMENSIONS)))),
    )
}
