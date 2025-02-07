// app.rs

use crate::asset::{AssetTrait, AssetType};
use crate::models::Asset;
use crate::plot_utils::create_plot_line;
use crate::{Loan, Portfolio, RealEstate};
use chrono::{NaiveDate, TimeZone, Utc};
use eframe::egui;
use egui_plot::{Legend, Plot, PlotPoints};

pub struct WealthTrackerApp {
    label: String,
    portfolio: Portfolio,
    ui_assets: Vec<Asset>,
    selected_asset_type: AssetType,
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
        };
        let rental_property = RealEstate {
            name: "Rental Property".to_owned(),
            value: -500000.0,
            rate_per_year: 3.0,
            acquisition_date: today,
        };
        let house_loan = Loan {
            name: "House Loan".to_owned(),
            value: 500000.0,
            rate_per_year: 7.0,
            acquisition_date: today,
            monthly_principal: 3000.0,
        };
        portfolio.add_asset(Asset::RealEstate(primary_residence));
        portfolio.add_asset(Asset::RealEstate(rental_property));
        portfolio.add_asset(Asset::Loan(house_loan));

        let ui_assets = vec![Asset::RealEstate(RealEstate::default())];
        Self {
            label: "Wealth Tracker".to_owned(),
            portfolio,
            ui_assets,
            selected_asset_type: AssetType::RealEstate,
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
        const INTERVAL_DAYS: i64 = 1;
        egui::SidePanel::left("left").show(ctx, |ui| {
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
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.label);
            // Plot the portfolio value over time.
            let start_date = Utc::now().date_naive();
            let end_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
            let mut lines = Vec::new();
            for asset in &self.portfolio.assets {
                let line = create_plot_line(asset.clone(), start_date, end_date, INTERVAL_DAYS)
                    .name(asset.name());
                lines.push(line);
            }
            let max = self
                .portfolio
                .max_value(start_date, end_date, INTERVAL_DAYS);
            let min = self
                .portfolio
                .min_value(start_date, end_date, INTERVAL_DAYS);

            Plot::new("wealth_over_time")
                .legend(Legend::default())
                .include_y(0.0)
                .include_y(max)
                .include_y(min)
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
