use crate::asset::{AssetTrait, AssetType};
use crate::models::Asset;
use crate::plot_utils::create_plot_line;
use crate::{portfolio, Loan, Portfolio, RealEstate};
use chrono::{NaiveDate, TimeZone, Utc};
use eframe::egui;
use egui_plot::{Legend, Plot};
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ApplicationSettings {
    pub stroke_width: f32,
    pub interval_days: i64,
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            stroke_width: 2.0,
            interval_days: 1,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WealthTrackerApp {
    label: String,
    portfolio: Portfolio,
    selected_asset_type: AssetType,
    application_settings: ApplicationSettings,
}

impl Default for WealthTrackerApp {
    fn default() -> Self {
        let mut portfolio = Portfolio::new();

        // Default acquisition date for new items: today.
        let today = Utc::now().date_naive();

        let primary_residence = RealEstate {
            name: "Primary Residence".to_owned(),
            value: 1000000.0,
            rate_per_year: 5.0,
            acquisition_date: today,
            uuid: Uuid::new_v4(),
            should_delete: false,
        };
        let rental_property = RealEstate {
            name: "Rental Property".to_owned(),
            value: -500000.0,
            rate_per_year: 3.0,
            acquisition_date: today,
            uuid: Uuid::new_v4(),
            should_delete: false,
        };
        let house_loan = Loan {
            name: "House Loan".to_owned(),
            value: 500000.0,
            rate_per_year: 7.0,
            acquisition_date: today,
            monthly_principal: 3000.0,
            uuid: Uuid::new_v4(),
            should_delete: false,
        };
        portfolio.add_asset(Asset::RealEstate(primary_residence));
        portfolio.add_asset(Asset::RealEstate(rental_property));
        portfolio.add_asset(Asset::Loan(house_loan));

        Self {
            label: "Wealth Tracker".to_owned(),
            portfolio,
            selected_asset_type: AssetType::RealEstate,
            application_settings: ApplicationSettings::default(),
        }
    }
}

impl WealthTrackerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for WealthTrackerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        const INTERVAL_DAYS: i64 = 1;
        egui::SidePanel::left("left").show(ctx, |ui| {
            ui.group(|ui| {
                egui::ComboBox::from_label("Add new asset")
                    .selected_text(format!("{:?}", self.selected_asset_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.selected_asset_type,
                            AssetType::RealEstate,
                            "Real Estate",
                        );
                        ui.selectable_value(&mut self.selected_asset_type, AssetType::Loan, "Loan");
                    });
                match self.selected_asset_type {
                    AssetType::RealEstate => {
                        if ui.button("Add Real Estate").clicked() {
                            self.portfolio
                                .add_asset(Asset::RealEstate(RealEstate::default()));
                        }
                    }
                    AssetType::Loan => {
                        if ui.button("Add Loan").clicked() {
                            self.portfolio.add_asset(Asset::Loan(Loan::default()));
                        }
                    }
                }
            });
            ui.separator();
            ui.heading("Edit Assets");
            let mut id_to_delete: Uuid = Uuid::nil();
            for asset in &mut self.portfolio.assets {
                let header_name = asset.name();
                egui::CollapsingHeader::new(header_name)
                    .id_salt(asset.uuid())
                    .show(ui, |ui| {
                        asset.ui_edit(ui);
                        if asset.should_delete() {
                            id_to_delete = asset.uuid();
                        }
                    });
            }
            self.portfolio.delete_asset(id_to_delete);
            ui.collapsing("Application settings", |ui| {
                egui::global_theme_preference_buttons(ui);
                ui.horizontal(|ui| {
                    ui.label("Stroke width:");
                    ui.add(egui::Slider::new(
                        &mut self.application_settings.stroke_width,
                        0.0..=10.0,
                    ));
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.label);
            // Plot the portfolio value over time.
            let start_date = Utc::now().date_naive();
            let end_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
            let mut lines = Vec::new();
            for asset in &self.portfolio.assets {
                let line = create_plot_line(asset.clone(), start_date, end_date, INTERVAL_DAYS)
                    .name(asset.name())
                    .width(self.application_settings.stroke_width);
                lines.push(line);
            }
            let (max, min) = if self.portfolio.assets.is_empty() {
                (0.0, 0.0)
            } else {
                (
                    self.portfolio
                        .max_value(start_date, end_date, INTERVAL_DAYS),
                    self.portfolio
                        .min_value(start_date, end_date, INTERVAL_DAYS),
                )
            };

            Plot::new("wealth_over_time")
                .legend(Legend::default())
                .include_y(0.0)
                .include_y(max)
                .include_y(min)
                .clamp_grid(true)
                .x_axis_formatter(|x, _range| {
                    let timestamp = x.value as i64;
                    let date = Utc.timestamp_opt(timestamp, 0).unwrap();
                    date.format("%Y-%m-%d").to_string()
                })
                .show(ui, |plot_ui| {
                    for line in lines {
                        plot_ui.line(line);
                    }
                });
        });
    }
}
