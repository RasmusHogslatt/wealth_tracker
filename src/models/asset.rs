use crate::models::instrument::FinancialInstrument;
use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Asset {
    pub name: String,
    pub initial_value: f32,
    pub value_change_per_year: f32, // e.g., 0.05 for 5% per year (positive for growth, negative for depreciation)
    pub acquisition_date: NaiveDate,
}

impl FinancialInstrument for Asset {
    fn value_on(&self, date: NaiveDate) -> f32 {
        let years_elapsed = (date - self.acquisition_date).num_days() as f32 / 365.25;
        // Compute the new value. For positive rate_of_change, the asset's value increases.
        // For negative rate_of_change, the asset's value decreases.
        let new_value = self.initial_value * (1.0 + self.value_change_per_year * years_elapsed);
        new_value.max(0.0)
    }

    fn acquisition_date(&self) -> NaiveDate {
        self.acquisition_date
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            name: "Asset".to_owned(),
            initial_value: 0.0,
            value_change_per_year: 0.0,
            acquisition_date: chrono::Utc::now().date_naive(),
        }
    }
}
