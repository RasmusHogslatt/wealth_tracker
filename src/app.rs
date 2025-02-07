// app.rs

use crate::models::{Asset, Investment, Liability};
use crate::plot_utils::create_line_for_instrument;
use crate::{instrument, Portfolio};
use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

/// Enum for selecting the type of financial item to add.
#[derive(PartialEq)]
enum NewItemType {
    Asset,
    Liability,
    Investment,
}

pub struct WealthTrackerApp {
    label: String,
    portfolio: Portfolio,
    // New fields for adding a financial item:
    new_item_type: NewItemType,
    new_item_name: String,
    new_item_initial_value: f32,
    new_item_rate: f32,
    new_item_year: i32,
    new_item_month: u32,
    new_item_day: u32,
}

impl Default for WealthTrackerApp {
    fn default() -> Self {
        let mut portfolio = Portfolio::new();
        // Create a sample asset for demonstration.
        let asset = Asset {
            name: "Car".to_owned(),
            initial_value: 30000.0,
            value_change_per_year: -1.15,
            acquisition_date: Utc::now().date_naive(),
        };
        portfolio.add_instrument(Box::new(asset));

        // Default acquisition date for new items: today.
        let today = Utc::now().date_naive();

        Self {
            label: "Wealth Tracker".to_owned(),
            portfolio,
            new_item_type: NewItemType::Asset,
            new_item_name: "".to_owned(),
            new_item_initial_value: 0.0,
            new_item_rate: 0.0,
            new_item_year: today.year(),
            new_item_month: today.month(),
            new_item_day: today.day(),
        }
    }
}

impl WealthTrackerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for WealthTrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.label);

            // Collapsible section for adding a new financial item.
            ui.collapsing("Add New Financial Item", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    egui::ComboBox::from_id_source("new_item_type")
                        .selected_text(match self.new_item_type {
                            NewItemType::Asset => "Asset",
                            NewItemType::Liability => "Liability",
                            NewItemType::Investment => "Investment",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.new_item_type,
                                NewItemType::Asset,
                                "Asset",
                            );
                            ui.selectable_value(
                                &mut self.new_item_type,
                                NewItemType::Liability,
                                "Liability",
                            );
                            ui.selectable_value(
                                &mut self.new_item_type,
                                NewItemType::Investment,
                                "Investment",
                            );
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.new_item_name);
                });

                ui.horizontal(|ui| {
                    ui.label("Initial Value:");
                    ui.add(egui::DragValue::new(&mut self.new_item_initial_value));
                });

                ui.horizontal(|ui| {
                    let rate_label = match self.new_item_type {
                        NewItemType::Asset => "Yearly value rate of change:",
                        NewItemType::Liability => "Interest Rate:",
                        NewItemType::Investment => "Appreciation Rate:",
                    };
                    ui.label(rate_label);
                    ui.add(egui::DragValue::new(&mut self.new_item_rate));
                });

                ui.horizontal(|ui| {
                    ui.label("Acquisition Date:");
                    ui.label("Year:");
                    ui.add(egui::DragValue::new(&mut self.new_item_year));
                    ui.label("Month:");
                    ui.add(egui::DragValue::new(&mut self.new_item_month));
                    ui.label("Day:");
                    ui.add(egui::DragValue::new(&mut self.new_item_day));
                });

                if ui.button("Add Item").clicked() {
                    if let Some(date) = NaiveDate::from_ymd_opt(
                        self.new_item_year,
                        self.new_item_month,
                        self.new_item_day,
                    ) {
                        match self.new_item_type {
                            NewItemType::Asset => {
                                let asset = Asset {
                                    name: self.new_item_name.clone(),
                                    initial_value: self.new_item_initial_value,
                                    value_change_per_year: self.new_item_rate,
                                    acquisition_date: date,
                                };
                                self.portfolio.add_instrument(Box::new(asset));
                            }
                            NewItemType::Liability => {
                                let liability = Liability {
                                    name: self.new_item_name.clone(),
                                    principal: self.new_item_initial_value,
                                    interest_rate: self.new_item_rate,
                                    acquisition_date: date,
                                };
                                self.portfolio.add_instrument(Box::new(liability));
                            }
                            NewItemType::Investment => {
                                let investment = Investment {
                                    name: self.new_item_name.clone(),
                                    initial_value: self.new_item_initial_value,
                                    appreciation_rate: self.new_item_rate,
                                    acquisition_date: date,
                                };
                                self.portfolio.add_instrument(Box::new(investment));
                            }
                        }
                        // Optionally, clear/reset the input fields after adding.
                        self.new_item_name.clear();
                        self.new_item_initial_value = 0.0;
                        self.new_item_rate = 0.0;
                    } else {
                        ui.label("Invalid date entered!");
                    }
                }
            });

            let has_instruments = match self.portfolio.instruments.is_empty() {
                true => {
                    ui.label("No financial items added yet.");
                    false
                }
                false => {
                    ui.label("Financial Items:");
                    true
                }
            };

            // Plot the portfolio value over time.
            let start_date = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
            let end_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();

            Plot::new("wealth_over_time")
                .legend(Legend::default())
                .height(200.0)
                .x_axis_formatter(|x, _range| {
                    let timestamp = x.value as i64;
                    let date = Utc.timestamp_opt(timestamp, 0).unwrap();
                    date.format("%Y-%m-%d").to_string()
                })
                .show(ui, |plot_ui| {
                    match has_instruments {
                        true => {
                            for instrument in self.portfolio.instruments.iter() {
                                let line = create_line_for_instrument(
                                    instrument.as_ref(),
                                    start_date,
                                    end_date,
                                    1,
                                );
                                plot_ui.line(line);
                            }
                        }
                        false => {}
                    }
                    // Add more lines
                });
        });
    }
}
