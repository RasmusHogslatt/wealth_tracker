use crate::{asset::AssetTrait, plot_utils::get_value_points_for_asset, Asset};
use chrono::NaiveDate;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Portfolio {
    pub assets: Vec<Asset>,
}

impl Portfolio {
    pub fn new() -> Self {
        Self { assets: Vec::new() }
    }

    pub fn add_asset(&mut self, asset: Asset) {
        self.assets.push(asset);
    }

    pub fn total_value(&self, date: NaiveDate) -> f32 {
        self.assets.iter().map(|asset| asset.value(date)).sum()
    }

    pub fn max_value(&self, start_date: NaiveDate, end_date: NaiveDate, interval_days: i64) -> f32 {
        // find maximum value the portfolio has as as the sum of all assets for given points
        let mut values: Vec<(NaiveDate, f32)> = Vec::new();
        for asset in &self.assets {
            let points: Vec<(NaiveDate, f32)> =
                get_value_points_for_asset(&asset, start_date, end_date, interval_days);
            // map points to values
            for (index, value) in points.iter().enumerate() {
                if values.len() <= index {
                    values.push((value.0, value.1));
                } else {
                    values[index].1 += value.1;
                }
            }
        }
        let mut max = 0.0;
        for value in values {
            if value.1 > max {
                max = value.1;
            }
        }
        max
    }

    pub fn min_value(&self, start_date: NaiveDate, end_date: NaiveDate, interval_days: i64) -> f32 {
        // find minimum value the portfolio has as as the sum of all assets for given points
        let mut values: Vec<(NaiveDate, f32)> = Vec::new();
        for asset in &self.assets {
            let points: Vec<(NaiveDate, f32)> =
                get_value_points_for_asset(&asset, start_date, end_date, interval_days);
            // map points to values
            for (index, value) in points.iter().enumerate() {
                if values.len() <= index {
                    values.push((value.0, value.1));
                } else {
                    values[index].1 += value.1;
                }
            }
        }
        let mut min = f32::MAX;
        for value in values {
            if value.1 < min {
                min = value.1;
            }
        }
        min
    }
}
