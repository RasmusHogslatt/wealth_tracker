// models/liability.rs

use crate::models::instrument::FinancialInstrument;
use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Liability {
    pub name: String,
    pub principal: f32,
    pub interest_rate: f32, // e.g., 0.05 for 5% per year
    pub acquisition_date: NaiveDate,
}

impl FinancialInstrument for Liability {
    fn value_on(&self, date: NaiveDate) -> f32 {
        let years_elapsed = (date - self.acquisition_date).num_days() as f32 / 365.25;
        // Here we assume the liability grows with interest;
        // by returning a negative number, you can subtract it from asset value.
        -self.principal * (1.0 + self.interest_rate).powf(years_elapsed)
    }

    fn acquisition_date(&self) -> NaiveDate {
        self.acquisition_date
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Default for Liability {
    fn default() -> Self {
        Self {
            name: "Liability".to_owned(),
            principal: 0.0,
            interest_rate: 0.0,
            acquisition_date: chrono::Utc::now().date_naive(),
        }
    }
}
