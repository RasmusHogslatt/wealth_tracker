// models/investment.rs

use crate::models::instrument::FinancialInstrument;
use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Investment {
    pub name: String,
    pub initial_value: f32,
    pub appreciation_rate: f32, // e.g., 0.07 for 7% per year
    pub acquisition_date: NaiveDate,
}

impl FinancialInstrument for Investment {
    fn value_on(&self, date: NaiveDate) -> f32 {
        let years_elapsed = (date - self.acquisition_date).num_days() as f32 / 365.25;
        self.initial_value * (1.0 + self.appreciation_rate).powf(years_elapsed)
    }

    fn acquisition_date(&self) -> NaiveDate {
        self.acquisition_date
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Default for Investment {
    fn default() -> Self {
        Self {
            name: "Investment".to_owned(),
            initial_value: 0.0,
            appreciation_rate: 0.0,
            acquisition_date: chrono::Utc::now().date_naive(),
        }
    }
}
