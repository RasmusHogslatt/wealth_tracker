// // src/plot_utils.rs

use chrono::{Duration, NaiveDate};
use egui_plot::{Line, PlotPoints};

use crate::{asset::AssetTrait, Asset};

pub fn get_value_points_for_asset(
    asset: &Asset,
    start_date: NaiveDate,
    end_date: NaiveDate,
    interval_days: i64,
) -> Vec<(NaiveDate, f32)> {
    let mut current_date: NaiveDate = start_date;
    let mut data_points: Vec<(NaiveDate, f32)> = Vec::new();

    while current_date <= end_date {
        let value = asset.value(current_date);
        data_points.push((current_date, value));
        current_date += Duration::days(interval_days);
    }

    data_points
}

pub fn create_plot_line(
    asset: Asset,
    start_date: NaiveDate,
    end_date: NaiveDate,
    interval_days: i64,
) -> Line {
    let data_points = get_value_points_for_asset(&asset, start_date, end_date, interval_days);

    let plot_points: Vec<[f64; 2]> = data_points
        .into_iter()
        .map(|(date, value)| {
            // Convert the date into a UTC timestamp (at midnight) and then to f64.
            let timestamp = date
                .and_hms_opt(0, 0, 0)
                .expect("Invalid time")
                .and_utc()
                .timestamp() as f64;
            [timestamp, value as f64]
        })
        .collect();

    Line::new(PlotPoints::new(plot_points))
}
