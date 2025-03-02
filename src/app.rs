use crate::asset::{AssetTrait, AssetType};
use crate::models::Asset;
use crate::plot_utils::{create_plot_line, create_portfolio_plot_line};
use crate::{Cash, Loan, Portfolio, RealEstate, Tradable};
use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use eframe::egui;
use egui_plot::{Legend, Plot};
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ApplicationSettings {
    pub stroke_width: f32,
    pub interval_days: i64,
    pub end_date: (i32, u32),
    pub currency: String,
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            stroke_width: 2.0,
            interval_days: 45,
            end_date: (Utc::now().date_naive().year() + 30, 1),
            currency: CURRENCY_SYMBOLS[0].to_string(),
        }
    }
}
const CURRENCY_SYMBOLS: [&str; 18] = [
    "USD", "GBP", "EUR", "SEK", "JPY", "AUD", "CAD", "CHF", "CNY", "HKD", "NZD", "SGD", "MYR",
    "THB", "PHP", "IDR", "KRW", "CZK",
];
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
            should_delete: false,
            ..Default::default()
        };
        let rental_property = RealEstate {
            name: "Rental Property".to_owned(),
            value: -500000.0,
            rate_per_year: 3.0,
            acquisition_date: today,
            should_delete: false,
            ..Default::default()
        };
        let house_loan = Loan {
            name: "House Loan".to_owned(),
            value: 500000.0,
            rate_per_year: 7.0,
            acquisition_date: today,
            principal_payment: 1000.0,
            principal_frequency: crate::models::assets::tradable::ContributionFrequency::Monthly,
            should_delete: false,
            ..Default::default()
        };
        let stocks = Tradable {
            name: "Stocks".to_owned(),
            value: 1000.0,
            rate_per_year: 8.0,
            acquisition_date: today,
            contribution: 100.0,
            ..Default::default()
        };
        portfolio.add_asset(Asset::RealEstate(primary_residence));
        portfolio.add_asset(Asset::RealEstate(rental_property));
        portfolio.add_asset(Asset::Loan(house_loan));
        portfolio.add_asset(Asset::Tradable(stocks));

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
        egui_material_icons::initialize(&cc.egui_ctx);
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
        egui::SidePanel::left("left").show(ctx, |ui| {
            ui.group(|ui| {
                ui.heading("Add Assets");
                egui::ComboBox::from_label("Select asset type")
                    .selected_text(format!("{:?}", self.selected_asset_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.selected_asset_type,
                            AssetType::RealEstate,
                            "Real Estate",
                        );
                        ui.selectable_value(&mut self.selected_asset_type, AssetType::Loan, "Loan");
                        ui.selectable_value(
                            &mut self.selected_asset_type,
                            AssetType::Tradable,
                            "Tradable",
                        );
                        ui.selectable_value(&mut self.selected_asset_type, AssetType::Cash, "Cash");
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
                    AssetType::Tradable => {
                        if ui.button("Add Tradable").clicked() {
                            self.portfolio
                                .add_asset(Asset::Tradable(Tradable::default()));
                        }
                    }
                    AssetType::Cash => {
                        if ui.button("Add Cash").clicked() {
                            self.portfolio.add_asset(Asset::Cash(Cash::default()));
                        }
                    }
                }
            });
            ui.separator();
            ui.collapsing("Application settings", |ui| {
                egui::global_theme_preference_buttons(ui);
                ui.horizontal(|ui| {
                    ui.label("Stroke width:");
                    ui.add(egui::Slider::new(
                        &mut self.application_settings.stroke_width,
                        0.0..=10.0,
                    ));
                });
                ui.horizontal(|ui| {
                    ui.label("Resolution:");
                    ui.add(egui::Slider::new(
                        &mut self.application_settings.interval_days,
                        1..=365,
                    ));
                });
                ui.horizontal(|ui| {
                    ui.label("End date:");
                    ui.horizontal(|ui| {
                        ui.label("Year");
                        ui.add(egui::Slider::new(
                            &mut self.application_settings.end_date.0,
                            Utc::now().year()..=Utc::now().year() + 50,
                        ));
                    });

                    let start_month = if self.application_settings.end_date.0 == Utc::now().year() {
                        let next_month = Utc::now().month() + 1;
                        if next_month > 12 {
                            1
                        } else {
                            next_month
                        }
                    } else {
                        1
                    };

                    ui.horizontal(|ui| {
                        ui.label("Month");
                        ui.add(egui::Slider::new(
                            &mut self.application_settings.end_date.1,
                            start_month..=12,
                        ));
                    });
                });
                egui::ComboBox::from_label("Currency")
                    .selected_text(self.application_settings.currency.clone())
                    .show_ui(ui, |ui| {
                        for currency in CURRENCY_SYMBOLS {
                            ui.selectable_value(
                                &mut self.application_settings.currency,
                                currency.to_string(),
                                currency,
                            );
                        }
                    });
            });
            ui.separator();
            ui.heading("Assets");
            egui::ScrollArea::new(true).show(ui, |ui| {
                let mut id_to_delete: Uuid = Uuid::nil();
                for asset in &mut self.portfolio.assets {
                    let header_name = asset.name();
                    let colored_header = egui::RichText::new(header_name).color(asset.color());
                    ui.horizontal(|ui| {
                        egui::CollapsingHeader::new(colored_header)
                            .id_salt(asset.uuid())
                            .show(ui, |ui| {
                                asset.ui_edit(ui, self.application_settings.currency.clone() + " ");
                                if asset.should_delete() {
                                    id_to_delete = asset.uuid();
                                }
                            });
                        if ui.button(egui_material_icons::icons::ICON_DELETE).clicked() {
                            id_to_delete = asset.uuid();
                        }
                    });
                }
                self.portfolio.delete_asset(id_to_delete);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.label);
            // Plot the portfolio value over time.
            let start_date = Utc::now().date_naive();
            let end_date = NaiveDate::from_ymd_opt(
                self.application_settings.end_date.0,
                self.application_settings.end_date.1,
                1,
            )
            .unwrap();
            let mut lines = Vec::new();
            for asset in &self.portfolio.assets {
                let line = create_plot_line(
                    asset.clone(),
                    start_date,
                    end_date,
                    self.application_settings.interval_days,
                )
                .name(asset.name())
                .width(self.application_settings.stroke_width)
                .color(asset.color());
                lines.push(line);
            }
            let portfolio_line = create_portfolio_plot_line(
                &self.portfolio,
                start_date,
                end_date,
                self.application_settings.interval_days,
            );
            lines.push(portfolio_line);
            let (max, min) = if self.portfolio.assets.is_empty() {
                (0.0, 0.0)
            } else {
                (
                    self.portfolio.max_value(
                        start_date,
                        end_date,
                        self.application_settings.interval_days,
                    ),
                    self.portfolio.min_value(
                        start_date,
                        end_date,
                        self.application_settings.interval_days,
                    ),
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
                    match Utc.timestamp_opt(timestamp, 0) {
                        chrono::LocalResult::Single(date) => date.format("%Y-%m-%d").to_string(),
                        _ => "Invalid date".to_owned(),
                    }
                })
                .show(ui, |plot_ui| {
                    for line in lines {
                        plot_ui.line(line);
                    }
                });
        });
    }
}
