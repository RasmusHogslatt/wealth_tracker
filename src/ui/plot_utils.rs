// src/plot_utils.rs

use crate::models::instrument::FinancialInstrument;
use chrono::{Duration, NaiveDate, TimeZone, Utc};
use egui_plot::{Line, PlotPoints};

/// Creates a plot line for a given financial instrument over the specified date range.
///
/// # Arguments
///
/// * `instrument` - A reference to a type implementing `FinancialInstrument`.
/// * `start_date` - The start date for plotting.
/// * `end_date` - The end date for plotting.
/// * `interval_days` - The number of days between each data point.
///
/// # Returns
///
/// An `egui_plot::Line` containing the computed plot points.
pub fn create_line_for_instrument(
    instrument: &dyn FinancialInstrument,
    start_date: NaiveDate,
    end_date: NaiveDate,
    interval_days: i64,
) -> Line {
    let mut current_date = start_date;
    let mut data_points = Vec::new();

    while current_date <= end_date {
        let value = instrument.value_on(current_date);
        data_points.push((current_date, value));
        current_date += Duration::days(interval_days);
    }

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

    Line::new(PlotPoints::new(plot_points)).name(instrument.name())
}
